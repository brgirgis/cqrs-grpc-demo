use cqrs_es2::{
    AggregateError,
    IAggregate,
};
use serde::{
    Deserialize,
    Serialize,
};

use super::super::{
    commands::BankAccountCommand,
    events::{
        BankAccountEvent,
        BankAccountOpened,
        CustomerDepositedMoney,
        CustomerWithdrewCash,
        CustomerWroteCheck,
    },
};

#[derive(Serialize, Deserialize)]
pub struct BankAccount {
    account_id: String,
    balance: f64,
}

impl IAggregate for BankAccount {
    type Command = BankAccountCommand;

    type Event = BankAccountEvent;

    fn aggregate_type() -> &'static str {
        "bank_account"
    }

    fn handle(
        &self,
        command: Self::Command,
    ) -> Result<Vec<Self::Event>, AggregateError> {
        match command {
            BankAccountCommand::OpenBankAccount(payload) => {
                let event_payload = BankAccountOpened {
                    account_id: payload.account_id,
                };

                Ok(vec![
                    BankAccountEvent::BankAccountOpened(
                        event_payload,
                    ),
                ])
            },
            BankAccountCommand::DepositMoney(payload) => {
                let balance = self.balance + payload.amount;

                let event_payload = CustomerDepositedMoney {
                    amount: payload.amount,
                    balance,
                };

                Ok(vec![
                    BankAccountEvent::CustomerDepositedMoney(
                        event_payload,
                    ),
                ])
            },
            BankAccountCommand::WithdrawMoney(payload) => {
                let balance = self.balance - payload.amount;

                if balance < 0_f64 {
                    return Err(AggregateError::new(
                        "funds not available",
                    ));
                }

                let event_payload = CustomerWithdrewCash {
                    amount: payload.amount,
                    balance,
                };

                Ok(vec![
                    BankAccountEvent::CustomerWithdrewCash(
                        event_payload,
                    ),
                ])
            },
            BankAccountCommand::WriteCheck(payload) => {
                let balance = self.balance - payload.amount;

                if balance < 0_f64 {
                    return Err(AggregateError::new(
                        "funds not available",
                    ));
                }

                let event_payload = CustomerWroteCheck {
                    check_number: payload.check_number,
                    amount: payload.amount,
                    balance,
                };

                Ok(vec![
                    BankAccountEvent::CustomerWroteCheck(
                        event_payload,
                    ),
                ])
            },
        }
    }

    fn apply(
        &mut self,
        event: &Self::Event,
    ) {
        match event {
            BankAccountEvent::BankAccountOpened(e) => {
                self.account_id = e.account_id.clone();
            },
            BankAccountEvent::CustomerDepositedMoney(e) => {
                self.balance = e.balance;
            },
            BankAccountEvent::CustomerWithdrewCash(e) => {
                self.balance = e.balance;
            },
            BankAccountEvent::CustomerWroteCheck(e) => {
                self.balance = e.balance;
            },
        }
    }
}

impl Default for BankAccount {
    fn default() -> Self {
        BankAccount {
            account_id: "".to_string(),
            balance: 0_f64,
        }
    }
}

impl Clone for BankAccount {
    fn clone(&self) -> Self {
        BankAccount {
            account_id: self.account_id.clone(),
            balance: self.balance,
        }
    }
}
