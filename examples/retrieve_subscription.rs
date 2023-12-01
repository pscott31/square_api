#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let subscription_id = "your subscription id";
    let response = client
        .retrieve_subscription(subscription_id)
        .include("your include")
        .await
        .unwrap();
    println!("{:#?}", response);
}