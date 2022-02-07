use cqrs_es::DomainEvent;
use ethcontract::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TokensEvent {
    TransferredFrom {
        from: Address,
        to: Address,
        ids: Vec<U256>,
        values: Vec<U256>,
    },
}

impl DomainEvent for TokensEvent {
    fn event_type(&self) -> &'static str {
        match self {
            Self::TransferredFrom { .. } => "TransferredFrom",
        }
    }

    fn event_version(&self) -> &'static str {
        "1.0"
    }
}
