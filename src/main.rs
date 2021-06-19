use std::net::TcpListener;

use zero2mail::configuration::get_configuration;
use zero2mail::startup::run;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to load configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    run(TcpListener::bind(address)?)?.await
}
