#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let card_id = "your card id";
    let response = client.retrieve_card(card_id).await.unwrap();
    println!("{:#?}", response);
}