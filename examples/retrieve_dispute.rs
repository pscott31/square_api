#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let dispute_id = "your dispute id";
    let response = client.retrieve_dispute(dispute_id).await.unwrap();
    println!("{:#?}", response);
}