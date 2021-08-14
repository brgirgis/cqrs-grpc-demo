mod arguments;
mod bank_account;
#[path = "../proto/bank_account_api.rs"]
mod bank_account_api;
mod cqrs;
mod tests;

// arguments
use arguments::Arguments;
use structopt::StructOpt;

// service
use bank_account::BankAccountService;

// server
use bank_account_api::bank_account_server::BankAccountServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::from_args();

    let addr = format!("[::1]:{}", args.port)
        .parse()
        .unwrap();

    let account_service = BankAccountService::default();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(BankAccountServer::new(account_service))
        .serve(addr)
        .await?;
    Ok(())
}
