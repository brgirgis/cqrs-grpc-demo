use tonic::{
    Request,
    Response,
    Status,
};

//use cqrs_es2::AggregateError;

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

//use super::super::framework::bank_account_repo;

#[derive(Default)]
pub struct BankAccountService {}

#[tonic::async_trait]
impl BankAccount for BankAccountService {
    async fn open_account(
        &self,
        request: Request<OpenBankAccountRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        // let cmd = request.get_ref();

        // let mut cqrs = bank_account_repo();
        // cqrs.execute(aggregate_id, cmd)
        Ok(Response::new(CommandResponse {
            is_successful: true,
        }))
    }

    async fn deposit_money(
        &self,
        request: Request<DepositMoneyRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        let _req = request.get_ref();

        Ok(Response::new(CommandResponse {
            is_successful: true,
        }))
    }

    async fn withdraw_money(
        &self,
        request: Request<WithdrawMoneyRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        let _req = request.get_ref();

        Ok(Response::new(CommandResponse {
            is_successful: true,
        }))
    }

    async fn write_check(
        &self,
        request: Request<WriteCheckRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        let _req = request.get_ref();

        Ok(Response::new(CommandResponse {
            is_successful: true,
        }))
    }

    async fn get_account_summary(
        &self,
        request: Request<BankAccountQueryRequest>,
    ) -> Result<Response<BankAccountSummaryResponse>, Status> {
        println!("{:?}", request);

        let _req = request.get_ref();

        Ok(Response::new(
            BankAccountSummaryResponse {
                account_id: "".to_string(),
                balance: 0.0,
                written_checks: vec![0; 0],
            },
        ))
    }
}
