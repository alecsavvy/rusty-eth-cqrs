use ethcontract::common::Bytecode;
use ethcontract::futures::StreamExt;
use ethcontract::prelude::*;
use ethcontract::web3::types::TransactionRequest;
use uuid::Uuid;

use crate::error::AppError;

ethcontract::contract!("./contracts/build/contracts/Tokens.json");

pub struct TokensContract {
    web3: Web3<Http>,
    instance: Tokens,
}

impl TokensContract {
    pub async fn new() -> Result<Self, AppError> {
        let http = Http::new("http://localhost:8545")?;
        let web3 = Web3::new(http);

        let instance = Tokens::builder(&web3)
            .gas(U256::max_value())
            .deploy()
            .await?;

        Ok(Self { web3, instance })
    }

    pub async fn mint(&self, from: Uuid, id: U256) -> Result<(), AppError> {
        let mut mints = self.instance.events().transfer_single().stream().boxed();

        futures::join! {
            async {
                self.instance.mint(Address::random().into(), id, U256::from(1 as u64), Bytes::default()).send().await.expect("mint failed");
            },
            async {
                let mint = mints.next().await.expect("no more events").expect("error querying event").transfer_single().expect("expected transfer single event");
                println!("found transfer single event {:#?}", mint);
            }
        };

        Ok(())
    }
}
