use actix_web::App;
use chrono::NaiveDate;
use mini_payment::configuration::get_configuration;
use mini_payment::local_database::Database;
use mini_payment::service::Application;
use mini_payment::user::{CountryName, DocumentNumber, User, UserName};
use std::error::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration file");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
