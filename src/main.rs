mod config;
mod routes;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,
};
use std::net::Ipv4Addr;

static PORT: &str = "PORT";
static DATABASE_URL: &str = "DATABASE_URL";

pub type AppResult<T> = Result<T, AppError>;
pub type AppJsonResult<T> = AppResult<Json<T>>;
pub enum AppError {
    DatabaseError(String),
    NotFound,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load environment variables into memory");

    let app = Router::new();
    let port = std::env::var(PORT)
        .unwrap_or_else(|_| String::from("3000"))
        .parse::<u16>()
        .expect("Failed to parse Port.");
    let addr = (Ipv4Addr::UNSPECIFIED, port).into();

    println!("App starting on http://0.0.0.0:{}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::DatabaseError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
        };

        status.into_response()
    }
}
