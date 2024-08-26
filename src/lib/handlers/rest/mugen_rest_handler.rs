use std::sync::Arc;

use axum::{http, Json};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::handlers::rest::requests::run_game_request::RunGameRequest;
use crate::handlers::rest::responses::run_game_response::RunGameResponse;
use crate::services::mugen_service::MugenService;

pub async fn start(State(state): State<Arc<dyn MugenService + Send + Sync>>
                   , Json(request): Json<RunGameRequest>) -> Response {
    return match state.start(request.blue_fighter(), request.red_fighter()) {
        Ok(winner) => {
            log::info!("the winner is... {}", winner);
            (StatusCode::OK, Json(RunGameResponse::new(winner.to_string()))).into_response()
        }
        Err(err) => {
            log::error!("{}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    };
}

