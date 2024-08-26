use axum::http;

use lib::handlers::rest::requests::run_game_request::RunGameRequest;
use lib::handlers::rest::responses::run_game_response::RunGameResponse;

use crate::setup::HTTP_SERVER_URL;

#[tokio::test]
async fn should_returns_winner() {
    let client = reqwest::Client::new();
    let response = client.post(format!("{}{}", HTTP_SERVER_URL, "/run"))
        .json(&RunGameRequest::new("FreezaZ2".to_string(), "GokuZ2".to_string()))
        .send()
        .await.unwrap()
        .json::<RunGameResponse>()
        .await.unwrap();


    assert_eq!(true, response.winner().eq("FreezaZ2")
        || response.winner().eq("GokuZ2"));
}

#[tokio::test]
async fn should_not_run_when_not_valid_character() {
    let client = reqwest::Client::new();
    let response = client.post(format!("{}{}", HTTP_SERVER_URL, "/run"))
        .json(&RunGameRequest::new("INVALID_CHARACTER".to_string(), "INVALID_CHARACTER_2".to_string()))
        .send()
        .await.unwrap();

    assert_eq!(http::StatusCode::INTERNAL_SERVER_ERROR, response.status());
}
