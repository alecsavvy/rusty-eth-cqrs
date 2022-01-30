use axum::{routing::get, Router};
use std::net::SocketAddr;

mod error;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    let app = Router::new().route("/health", get(health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server couldn't bind");
    Ok(())
}

// health check
async fn health() -> &'static str {
    "Up!"
}
