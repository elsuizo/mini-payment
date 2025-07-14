use crate::configuration::ServiceSettings;
use crate::local_database::Database;
use crate::routes::{
    client_creation, decrease_balance, get_balance, increase_balance, store_balances,
};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: ServiceSettings) -> Result<Self, anyhow::Error> {
        let host = configuration.application.host;
        let port_config = configuration.application.port;
        let listener = TcpListener::bind(format!("{}:{}", host, port_config))?;
        // NOTE(elsuizo: 2024-10-17): obtenemos el puerto que nos ha asignado el OS
        let port = listener.local_addr().unwrap().port();
        let database = Arc::new(Mutex::new(Database::new()));
        let server = run(listener, database).await?;
        Ok(Self { port, server })
    }

    pub fn get_port_number(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

/// server main function
pub async fn run(
    listener: TcpListener,
    database: Arc<Mutex<Database>>,
) -> Result<Server, anyhow::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/new_client", web::post().to(client_creation))
            .route("/new_credit_transaction", web::post().to(increase_balance))
            .route("/new_debit_transaction", web::post().to(decrease_balance))
            .route("/store_balances", web::post().to(store_balances))
            .route("/client_balance", web::get().to(get_balance))
            // NOTE(elsuizo: 2025-07-12): clone a Arc is cheap :)
            .app_data(web::Data::new(database.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
