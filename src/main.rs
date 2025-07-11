use mini_payment::configuration::get_configuration;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration file");

    println!("{configuration:?}");
    println!("Hello, world!");
}
