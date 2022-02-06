use std::time::Duration;

use axum::handler::future;
use ethcontract::{Http, Web3};
use futures::StreamExt;
use tokio::time::interval;

use crate::contract::TokensContract;

/**
 * Entity is what ties the contract and store together.
 * They function independently of each other.
 */
#[derive(Debug, Clone)]
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
        let event_stream = self.contract.event_stream();
        let (mut transfer_singles, mut transfer_batches) = event_stream;
        loop {
            listening_interval.tick().await;
            println!("reading block for events");

            let (tse, tbe) = tokio::join!(transfer_singles.next(), transfer_batches.next());

            if let Some(Ok(tse)) = tse {
                println!("found transfer single event {:#?}", tse);
            }

            if let Some(Ok(tbe)) = tbe {
                println!("found transfer batch event {:#?}", tbe);
            }

            // TODO: call callback on each interval if applicable
        }
    }
}
