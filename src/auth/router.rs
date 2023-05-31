use axum::{Router, routing::get, Json, extract::State, debug_handler};
use serde::{Deserialize, Serialize};
use crate::{AppState, AppJsonResult};

#[derive(Deserialize, Serialize)]
struct AuthResponse {
    message: String,
    status_code: i32
}

pub fn new() -> Router<AppState> {
    Router::new().route("/", get(login))
}

async fn login(State(state): State<AppState>) -> AppJsonResult<AuthResponse> {

    Ok(Json::from(AuthResponse{message: "OK".to_string(), status_code:200}))
}
