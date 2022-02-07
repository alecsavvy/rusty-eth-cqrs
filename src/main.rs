use std::{net::SocketAddr, sync::Arc};

use api::routes::{admin_routes, routes, State};
use ethcontract::{Http, Web3};
use repositories::token_repository::TokenRepository;

mod api;
mod contract;
mod repositories;
mod store;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let admin_addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let http = Http::new("http://127.0.0.1:8545").expect("transport failed");
    let web3 = Web3::new(http);

    let tokens_entity = TokenRepository::new(web3).await;

    let state = Arc::new(State {
        token_repository: tokens_entity.clone(),
    });

    let public_routes = routes(state.clone());
    let public_server = axum::Server::bind(&addr).serve(public_routes.into_make_service());

    let admin_routes = admin_routes(state);
    let admin_server = axum::Server::bind(&admin_addr).serve(admin_routes.into_make_service());

    let event_stream = tokens_entity.event_listener();

    // run all tasks concurrently
    let (_admin, _public, _es) = tokio::join!(admin_server, public_server, event_stream);
}
