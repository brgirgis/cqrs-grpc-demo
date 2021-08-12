use cqrs_es2::DomainEvent;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq
)]
pub enum BankAccountEvent {
    BankAccountOpened(BankAccountOpened),
    CustomerDepositedMoney(CustomerDepositedMoney),
    CustomerWithdrewCash(CustomerWithdrewCash),
    CustomerWroteCheck(CustomerWroteCheck),
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq
)]
pub struct BankAccountOpened {
    pub account_id: String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq
)]
pub struct CustomerDepositedMoney {
    pub amount: f64,
    pub balance: f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq
)]
pub struct CustomerWithdrewCash {
    pub amount: f64,
    pub balance: f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq
)]
pub struct CustomerWroteCheck {
    pub check_number: String,
    pub amount: f64,
    pub balance: f64,
}

impl DomainEvent for BankAccountEvent {}