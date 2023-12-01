#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let idempotency_key = "your idempotency key";
    let order_id = "your order id";
    let response = client
        .pay_order(idempotency_key, order_id)
        .order_version(1)
        .payment_ids(&["your payment ids"])
        .await
        .unwrap();
    println!("{:#?}", response);
}