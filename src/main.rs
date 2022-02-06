use std::net::SocketAddr;

use api::routes::{admin_routes, routes};

mod api;
mod contract;
mod entity;
mod store;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let admin_addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    let public_routes = routes();
    let public_server = axum::Server::bind(&addr).serve(public_routes.into_make_service());

    let admin_routes = admin_routes();
    let admin_server = axum::Server::bind(&admin_addr).serve(admin_routes.into_make_service());

    // run all tasks concurrently
    let (_admin, _public) = tokio::join!(admin_server, public_server);
}
