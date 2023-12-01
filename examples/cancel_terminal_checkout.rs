#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let checkout_id = "your checkout id";
    let response = client.cancel_terminal_checkout(checkout_id).await.unwrap();
    println!("{:#?}", response);
}