#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let order_ids = &["your order ids"];
    let response = client
        .batch_retrieve_orders(order_ids)
        .location_id("your location id")
        .await
        .unwrap();
    println!("{:#?}", response);
}