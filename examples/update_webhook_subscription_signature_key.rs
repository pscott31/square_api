#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let subscription_id = "your subscription id";
    let response = client
        .update_webhook_subscription_signature_key(subscription_id)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}