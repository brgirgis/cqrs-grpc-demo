use std::fmt::Debug;

use cqrs_es2::IDomainCommand;

#[derive(Debug, PartialEq)]
pub enum BankAccountCommand {
    OpenBankAccount(OpenBankAccount),
    DepositMoney(DepositMoney),
    WithdrawMoney(WithdrawMoney),
    WriteCheck(WriteCheck),
}

#[derive(Debug, PartialEq)]
pub struct OpenBankAccount {
    pub account_id: String,
}

#[derive(Debug, PartialEq)]
pub struct DepositMoney {
    pub amount: f64,
}

#[derive(Debug, PartialEq)]
pub struct WithdrawMoney {
    pub amount: f64,
}

#[derive(Debug, PartialEq)]
pub struct WriteCheck {
    pub check_number: String,
    pub amount: f64,
}

impl IDomainCommand for BankAccountCommand {}
