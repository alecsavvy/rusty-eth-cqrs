use super::routes::AppState;

#[allow(dead_code)]

pub async fn mint_nft(state: AppState) {
    let tokens_entity = &state.token_repository;
    tokens_entity.mint().await;
}

pub async fn get_nft(_state: AppState, _id: String) {}

pub async fn mint_currency(_state: AppState) {}
