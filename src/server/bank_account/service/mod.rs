use tonic::{
    Request,
    Response,
    Status,
};

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

#[derive(Default)]
pub struct BankAccountService {}

#[tonic::async_trait]
impl BankAccount for BankAccountService {
    async fn open_account(
        &self,
        request: Request<OpenBankAccountRequest>,
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
