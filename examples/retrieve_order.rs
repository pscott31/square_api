#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let order_id = "your order id";
    let response = client.retrieve_order(order_id).await.unwrap();
    println!("{:#?}", response);
}