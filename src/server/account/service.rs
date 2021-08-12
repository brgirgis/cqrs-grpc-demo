use tonic::{
    Request,
    Response,
    Status,
};

use crate::account_api::{
    account_server::Account,
    AccountQueryRequest,
    AccountSummaryResponse,
    CommandResponse,
    DepositMoneyRequest,
    OpenAccountRequest,
    WithdrawMoneyRequest,
    WriteCheckRequest,
};

#[derive(Default)]
pub struct AccountService {}

#[tonic::async_trait]
impl Account for AccountService {
    async fn open_account(
        &self,
        request: Request<OpenAccountRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        let req = request.get_ref();

        Ok(Response::new(CommandResponse {
            is_successful: true,
        }))
    }

    async fn deposit_money(
        &self,
        request: Request<DepositMoneyRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        let req = request.get_ref();

        Ok(Response::new(CommandResponse {
            is_successful: true,
        }))
    }

    async fn withdraw_money(
        &self,
        request: Request<WithdrawMoneyRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        let req = request.get_ref();

        Ok(Response::new(CommandResponse {
            is_successful: true,
        }))
    }

    async fn write_check(
        &self,
        request: Request<WriteCheckRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        println!("{:?}", request);

        let req = request.get_ref();

        Ok(Response::new(CommandResponse {
            is_successful: true,
        }))
    }

    async fn get_account_summary(
        &self,
        request: Request<AccountQueryRequest>,
    ) -> Result<Response<AccountSummaryResponse>, Status> {
        println!("{:?}", request);

        let req = request.get_ref();

        Ok(Response::new(AccountSummaryResponse {
            account_id: "".to_string(),
            balance: 0.0,
            written_checks: vec![0; 0],
        }))
    }
}
