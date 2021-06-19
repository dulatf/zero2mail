use reqwest::Client;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2mail::configuration::get_configuration;
#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute requrest.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2mail::startup::run(listener).expect("Failed to bind address.");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let server_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let client = Client::new();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", server_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions.");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let server_address = spawn_app();
    let client = Client::new();

    let test_cases = vec![
        ("name=foo", "missing the email"),
        ("email=foo%40bar.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &server_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The api did not fail with 400 when the payload was {}.",
            error_message
        );
    }
}
