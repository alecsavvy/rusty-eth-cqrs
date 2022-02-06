use std::time::Duration;

use ethcontract::{Http, Web3};
use tokio::time::interval;

use crate::contract::TokensContract;

/**
 * Entity is what ties the contract and store together.
 * They function independently of each other.
 */

pub struct TokensEntity {
    contract: TokensContract,
}

impl TokensEntity {
    pub async fn new(web3: Web3<Http>) -> Self {
        let contract = TokensContract::new(web3).await;
        Self { contract }
    }

    pub async fn event_listener(&self /* callback */) {
        let mut listening_interval = interval(Duration::from_secs(2));
        loop {
            listening_interval.tick().await;
            println!("reading block for events");
            // TODO: call callback on each interval if applicable
        }
    }
}
