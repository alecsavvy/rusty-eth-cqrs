use crate::repositories::token_repository::TokenRepository;
use axum::{
    extract::Extension,
    routing::{get, post},
    AddExtensionLayer, Router,
};
use std::sync::Arc;

use super::{tokens::*, user::*};

pub struct State {
    pub token_repository: TokenRepository,
    /* users_entity: UsersEntity */
}

pub type AppState = Extension<Arc<State>>;

pub fn routes(state: Arc<State>) -> Router {
    Router::new()
        .route("/user", post(create_user))
        .route("/user/:id", get(get_user))
        .route("/nft", post(mint_nft))
        .route("/nft/:id", get(get_nft))
        .route("/currency", post(mint_currency))
        .layer(AddExtensionLayer::new(state))
}

pub fn admin_routes(state: Arc<State>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .layer(AddExtensionLayer::new(state))
}

async fn health_check(_state: AppState) -> &'static str {
    "up"
}
