#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let dispute_id = "your dispute id";
    let response = client
        .list_dispute_evidence(dispute_id)
        .cursor("your cursor")
        .await
        .unwrap();
    println!("{:#?}", response);
}