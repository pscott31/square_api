#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let transfer_id = "your transfer id";
    let response = client.retrieve_inventory_transfer(transfer_id).await.unwrap();
    println!("{:#?}", response);
}