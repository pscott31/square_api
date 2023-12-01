#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let nonce = "your nonce";
    let response = client.retrieve_gift_card_from_nonce(nonce).await.unwrap();
    println!("{:#?}", response);
}