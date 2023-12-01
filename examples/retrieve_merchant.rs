#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let merchant_id = "your merchant id";
    let response = client.retrieve_merchant(merchant_id).await.unwrap();
    println!("{:#?}", response);
}