use ethcontract::{Address, U256};
/**
 * Events from a contract are serialized into commands.
 * These commands are persisted into the database.
 * */
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum TokensCommand {
    TransferFrom {
        operator: Address,
        from: Address,
        to: Address,
        ids: Vec<U256>,
        values: Vec<U256>,
    },
}
