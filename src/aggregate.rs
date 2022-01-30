use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::{commands::BankAccountCommand, events::BankAccountEvent};

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct BankAccount {
    opened: bool,
    balance: f64,
}

impl Aggregate for BankAccount {
    type Command = BankAccountCommand;

    type Event = BankAccountEvent;

    fn aggregate_type() -> &'static str {
        "account"
    }

    fn handle(&self, command: Self::Command) -> Result<Vec<Self::Event>, cqrs_es::AggregateError> {
        Ok(vec![])
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            BankAccountEvent::AccountOpened { .. } => self.opened = true,
            BankAccountEvent::CustomerDepositedMoney { balance, .. } => self.balance = balance,
            BankAccountEvent::CustomerWithdrewCash { balance, .. } => self.balance = balance,
            BankAccountEvent::CustomerWroteCheck { balance, .. } => self.balance = balance,
        }
    }
}
