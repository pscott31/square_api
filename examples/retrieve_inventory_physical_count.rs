#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let physical_count_id = "your physical count id";
    let response = client
        .retrieve_inventory_physical_count(physical_count_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}