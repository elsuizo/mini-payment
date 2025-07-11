use crate::configuration;
use crate::configuration::ServiceSettings;
use crate::routes::client_creation;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;

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
        let server = run(listener, host).await?;
        Ok(Self { port, server })
    }

    pub fn get_port_number(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

// TODO(elsuizo: 2025-07-10): better name maybe
pub async fn run(listener: TcpListener, base_url: String) -> Result<Server, anyhow::Error> {
    // TODO(elsuizo: 2025-07-11): the endpoints here
    let server =
        HttpServer::new(move || App::new().route("/new_client", web::post().to(client_creation)))
            .listen(listener)?
            .run();
    Ok(server)
}
