use ethcontract::{Address, Http, Web3};
use futures::StreamExt;

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

    pub async fn mint(&self) {
        let instance = &self.contract;
        instance
            .mint(Address::random(), vec![1.into()], vec![1.into()])
            .await;
    }

    pub async fn event_listener(&self /* callback */) {
        let mut batch_transfers = self.contract.event_stream();
        while let Some(Ok(event)) = batch_transfers.next().await {
            println!("found a transfer {:#?}", event);
        }
    }
}
