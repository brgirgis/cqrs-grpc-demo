use serde::{
    Deserialize,
    Serialize,
};

use cqrs_es2::{
    EventEnvelope,
    IQuery,
};

use cqrs_es2_sql::GenericQueryRepository;

use super::super::{
    aggregate::BankAccount,
    events::BankAccountEvent,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccountQuery {
    account_id: Option<String>,
    balance: f64,
    written_checks: Vec<String>,
}

impl IQuery<BankAccount> for BankAccountQuery {
    fn update(
        &mut self,
        event: &EventEnvelope<BankAccount>,
    ) {
        match &event.payload {
            BankAccountEvent::BankAccountOpened(payload) => {
                self.account_id = Some(payload.account_id.clone());
            },
            BankAccountEvent::CustomerDepositedMoney(payload) => {
                self.balance = payload.balance;
            },
            BankAccountEvent::CustomerWithdrewCash(payload) => {
                self.balance = payload.balance;
            },
            BankAccountEvent::CustomerWroteCheck(payload) => {
                self.balance = payload.balance;
                self.written_checks
                    .push(payload.check_number.clone())
            },
        }
    }
}

impl Default for BankAccountQuery {
    fn default() -> Self {
        BankAccountQuery {
            account_id: None,
            balance: 0_f64,
            written_checks: Default::default(),
        }
    }
}

pub type BankAccountQueryRepository =
    GenericQueryRepository<BankAccountQuery, BankAccount>;
