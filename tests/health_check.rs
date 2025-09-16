use std::net::TcpListener;

use reqwest;
use tokio;
use zero2prod::run;

/// Spin up an instance of our application
/// and returns its address (i.e. http://localhost:XXXX)
fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind the port.");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind the address");

    tokio::spawn(server);

    format!("http://localhost:{}", port)
}

#[actix_web::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute the request.");

    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.content_length(), Some(0));
}

#[actix_web::test]
async fn subscription_returns_a_200_for_valid_form_data() {
    // Arrange
    let data = "name=Yabets%20Zekaryas&email=yabetszkearyas07%40gmail.com";
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(data)
        .send()
        .await
        .expect("Failed to execute the request.");

    // Assert
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[actix_web::test]
async fn subscription_returns_a_400_for_invalid_form_data() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            response.status(),
            reqwest::StatusCode::BAD_REQUEST,
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
