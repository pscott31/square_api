#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let subscription_id = "your subscription id";
    let response = client
        .test_webhook_subscription(subscription_id)
        .event_type("your event type")
        .await
        .unwrap();
    println!("{:#?}", response);
}