use env_logger::Env;
use mini_payment::configuration::get_configuration;
use mini_payment::service::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO(elsuizo: 2025-07-13): esto deberia ser parte de la configuracion
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let configuration = get_configuration()?;
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
