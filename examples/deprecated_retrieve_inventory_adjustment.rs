#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let adjustment_id = "your adjustment id";
    let response = client
        .deprecated_retrieve_inventory_adjustment(adjustment_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}