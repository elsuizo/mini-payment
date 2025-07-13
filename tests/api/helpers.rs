use actix_web::App;
use uuid::Uuid;
use wiremock::MockServer;

use mini_payment::configuration::get_configuration;
use mini_payment::service::Application;

pub struct TestUser {
    pub client_name: String,
    pub bird_date: String,
    pub document_number: String,
    pub country: String,
}

impl TestUser {}

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub test_user: TestUser,
    pub api_client: reqwest::Client,
}

impl TestApp {}

/// con esta funcion lo que hacemos es crear una instancia de la app
pub async fn spawn_app(test_user: TestUser) -> TestApp {
    let email_server = MockServer::start().await;

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration");
        // usamos una base de datos diferente para cada caso de test
        // usamos un puerto del OS random
        c.application.port = 0;
        // usamos al server de test como API de email
        c
    };

    // Create and migrate the database

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");
    // obtenemos el port antes de spamear la aplicacion
    let address = format!("http://localhost:{}", application.get_port_number());
    let application_port = application.get_port_number();

    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let test_app = TestApp {
        address,
        port: application_port,
        test_user,
        api_client: client,
    };

    test_app
}
