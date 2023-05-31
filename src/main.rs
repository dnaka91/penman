mod auth;
mod config;
mod env;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,
};
use db::postgres::PostgresProvider;
use std::{net::Ipv4Addr, sync::Arc};
use anyhow::Result;
use crate::env::Env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load environment variables into memory");
    let env = Env::init().expect("Failed to load environment variables into memory");

    let psql = PostgresProvider::new(env.database_url)
        .await
        .expect("Failed to create Postgres Connection");

    let app_state = AppState { db: Arc::new(psql) };

    let addr = (Ipv4Addr::UNSPECIFIED, env.port).into();

    let app = Router::new()
        .nest("/auth", auth::router::new())
        .with_state(app_state);

    println!("App starting on http://0.0.0.0:{}", env.port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

pub type AppResult<T> = Result<T, AppError>;
pub type AppJsonResult<T> = AppResult<Json<T>>;
pub enum AppError {
    DatabaseError(String),
    NotFound,
}
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PostgresProvider>,
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
