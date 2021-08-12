mod account;
#[path = "../proto/account-api.rs"]
mod account_api;
mod arguments;
mod cqrs;

// arguments
use arguments::Arguments;
use structopt::StructOpt;

// service
use account::service::AccountService;

// server
use account_api::account_server::AccountServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::from_args();

    let addr = format!("[::1]:{}", args.port).parse().unwrap();

    let account_service = AccountService::default();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(AccountServer::new(account_service))
        .serve(addr)
        .await?;
    Ok(())
}
