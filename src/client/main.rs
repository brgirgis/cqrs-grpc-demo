#[path = "../proto/account-api.rs"]
mod account_api;

use account_api::{
    account_client::EvaluationClient,
    VanillaDealRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel =
        tonic::transport::Channel::from_static("http://[::1]:50051")
            .connect()
            .await?;

    let mut client = EvaluationClient::new(channel);

    let request = tonic::Request::new(VanillaDealRequest {
        underlying_market_name: String::from(", world!"),
        option_type: 0,
        strike_value: 0.0,
        time_to_maturity: 0.0,
        spot_price: 0.0,
        interest_rate: 0.0,
        volatility: 0.0,
    });

    // sending request and waiting for response
    let response = client
        .evaluate_vanilla_deal(request)
        .await?
        .into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
