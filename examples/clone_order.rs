#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let order_id = "your order id";
    let response = client
        .clone_order(order_id)
        .idempotency_key("your idempotency key")
        .version(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}