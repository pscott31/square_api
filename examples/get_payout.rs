#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let payout_id = "your payout id";
    let response = client.get_payout(payout_id).await.unwrap();
    println!("{:#?}", response);
}