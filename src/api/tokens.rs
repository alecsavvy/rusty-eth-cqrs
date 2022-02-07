use super::routes::AppState;

pub async fn mint_nft(state: AppState) {
    let tokens_entity = &state.tokens_entity;
    tokens_entity.mint().await;
}

pub async fn get_nft(_state: AppState, _id: String) {}

pub async fn mint_currency(_state: AppState) {}
