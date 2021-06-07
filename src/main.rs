use std::net::TcpListener;

use zero2mail::run;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener::bind("127.0.0.1:8080")?)?.await
}
