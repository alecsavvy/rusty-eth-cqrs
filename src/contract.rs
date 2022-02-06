/**
 * Rust interface for the tokens contract.
 */
use ethcontract::{Bytes, Http, Web3, H160, U256};

ethcontract::contract!("./contracts/build/contracts/Tokens.json");

#[derive(Debug, Clone)]
pub struct TokensContract {
    pub web3: Web3<Http>,
    pub instance: Tokens,
}

impl TokensContract {
    pub async fn new(web3: Web3<Http>) -> Self {
        let instance = Tokens::builder(&web3)
            .deploy()
            .await
            .expect("could not deploy tokens contract");

        Self { web3, instance }
    }

    pub async fn mint(&self, to: H160, ids: Vec<U256>, amounts: Vec<U256>) -> Self {
        let instance = &self.instance;
        instance
            .mint_batch(to, ids, amounts, Bytes::default())
            .send()
            .await
            .expect("mint failed");

        self.to_owned()
    }

    pub async fn transfer(&self, from: H160, to: H160, ids: Vec<U256>, amounts: Vec<U256>) -> Self {
        let instance = &self.instance;
        instance
            .safe_batch_transfer_from(from, to, ids, amounts, Bytes::default())
            .send()
            .await
            .expect("transfer failed");

        self.to_owned()
    }
}
