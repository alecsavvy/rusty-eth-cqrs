use axum::{
    routing::{get, post},
    Router,
};

use super::{tokens::*, user::*};

pub fn routes() -> Router {
    Router::new()
        .route("/user", post(create_user))
        .route("/user/:id", get(get_user))
        .route("/nft", post(mint_nft))
        .route("/nft/:id", get(get_nft))
        .route("/currency", post(mint_currency))
}

pub fn admin_routes() -> Router {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "up"
}
