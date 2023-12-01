#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let subscription_id = "your subscription id";
    let response = client.cancel_subscription(subscription_id).await.unwrap();
    println!("{:#?}", response);
}