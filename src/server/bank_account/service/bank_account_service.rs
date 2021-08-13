use std::collections::HashMap;

use tonic::{
    Request,
    Response,
    Status,
};

use cqrs_es2::AggregateError;

use crate::bank_account_api::{
    bank_account_server::BankAccount,
    BankAccountQueryRequest,
    BankAccountSummaryResponse,
    CommandResponse,
    DepositMoneyRequest,
    OpenBankAccountRequest,
    WithdrawMoneyRequest,
    WriteCheckRequest,
};

use super::super::framework::bank_account_repo;

#[derive(Default)]
pub struct BankAccountService {}

impl BankAccountService {
    fn process_command(
        &self,
        payload_type: &str,
        aggregate_id: &str,
        payload: String,
    ) -> Result<(), AggregateError> {
        let event_ser =
            format!("{{\"{}\":{}}}", payload_type, payload);
        let payload = match serde_json::from_str(event_ser.as_str()) {
            Ok(payload) => payload,
            Err(err) => {
                return Err(AggregateError::TechnicalError(
                    err.to_string(),
                ));
            },
        };
        let mut cqrs = bank_account_repo();
        let mut metadata = HashMap::new();
        metadata.insert(
            "time".to_string(),
            chrono::Utc::now().to_rfc3339(),
        );
        cqrs.execute_with_metadata(aggregate_id, payload, metadata)
    }
}

#[tonic::async_trait]
impl BankAccount for BankAccountService {
    async fn open_account(
        &self,
        request: Request<OpenBankAccountRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        let req = request.get_ref();

        match self.process_command(
            "OpenBankAccount",
            &req.account_id,
            format!(
                "{{\"account_id\": \"{}\"}}",
                &req.account_id
            ),
        ) {
            Ok(_) => {
                Ok(Response::new(CommandResponse {
                    is_successful: true,
                }))
            },
            Err(err) => {
                let err_payload = match &err {
                    AggregateError::UserError(e) => {
                        serde_json::to_string(e).unwrap()
                    },
                    AggregateError::TechnicalError(e) => e.clone(),
                };
                println!("ERROR: {}", &err_payload);
                Err(Status::unknown("err_payload"))
            },
        }
    }

    async fn deposit_money(
        &self,
        request: Request<DepositMoneyRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        let req = request.get_ref();

        match self.process_command(
            "DepositMoney",
            &req.account_id,
            format!("amount:{}", req.amount),
        ) {
            Ok(_) => {
                Ok(Response::new(CommandResponse {
                    is_successful: true,
                }))
            },
            Err(err) => {
                let err_payload = match &err {
                    AggregateError::UserError(e) => {
                        serde_json::to_string(e).unwrap()
                    },
                    AggregateError::TechnicalError(e) => e.clone(),
                };

                Err(Status::unknown(err_payload))
            },
        }
    }

    async fn withdraw_money(
        &self,
        request: Request<WithdrawMoneyRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        let req = request.get_ref();

        match self.process_command(
            "WithdrawMoney",
            &req.account_id,
            format!("amount:{}", req.amount),
        ) {
            Ok(_) => {
                Ok(Response::new(CommandResponse {
                    is_successful: true,
                }))
            },
            Err(err) => {
                let err_payload = match &err {
                    AggregateError::UserError(e) => {
                        serde_json::to_string(e).unwrap()
                    },
                    AggregateError::TechnicalError(e) => e.clone(),
                };

                Err(Status::unknown(err_payload))
            },
        }
    }

    async fn write_check(
        &self,
        request: Request<WriteCheckRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        let req = request.get_ref();

        match self.process_command(
            "WriteCheck",
            &req.account_id,
            format!(
                "amount:{}, check_number:{}",
                req.amount, req.check_number
            ),
        ) {
            Ok(_) => {
                Ok(Response::new(CommandResponse {
                    is_successful: true,
                }))
            },
            Err(err) => {
                let err_payload = match &err {
                    AggregateError::UserError(e) => {
                        serde_json::to_string(e).unwrap()
                    },
                    AggregateError::TechnicalError(e) => e.clone(),
                };

                Err(Status::unknown(err_payload))
            },
        }
    }

    async fn get_account_summary(
        &self,
        request: Request<BankAccountQueryRequest>,
    ) -> Result<Response<BankAccountSummaryResponse>, Status> {
        println!("{:?}", request);

        let req = request.get_ref();

        Ok(Response::new(
            BankAccountSummaryResponse {
                account_id: "".to_string(),
                balance: 0.0,
                written_checks: vec![0; 0],
            },
        ))
    }
}
