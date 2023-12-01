#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let idempotency_key = "your idempotency key";
    let response = client
        .cancel_payment_by_idempotency_key(idempotency_key)
        .await
        .unwrap();
    println!("{:#?}", response);
}