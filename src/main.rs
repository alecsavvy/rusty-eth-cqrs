use axum::{routing::get, Router};
use std::{net::SocketAddr, time::Duration};
use tokio::time::interval;

mod api;
mod contract;
mod entity;
mod store;

#[tokio::main]
async fn main() {
    // run all tasks concurrently
    // TODO: add error handling for each future
    let (_reader, _server) = tokio::join!(chain_reader(), web_server());
}

/**
 * Reads chain blocks for events on an interval.
 */
async fn chain_reader() {
    let mut reading_interval = interval(Duration::from_secs(3));
    loop {
        reading_interval.tick().await;
        println!("reading chain block");
    }
}

async fn web_server() {
    let app = Router::new().route("/", get(health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("could not stand up server");
}

async fn health() -> &'static str {
    println!("received request");
    "up"
}
