#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let gan = "your gan";
    let response = client.retrieve_gift_card_from_gan(gan).await.unwrap();
    println!("{:#?}", response);
}