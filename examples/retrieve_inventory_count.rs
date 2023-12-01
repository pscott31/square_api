#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let catalog_object_id = "your catalog object id";
    let response = client
        .retrieve_inventory_count(catalog_object_id)
        .cursor("your cursor")
        .location_ids("your location ids")
        .await
        .unwrap();
    println!("{:#?}", response);
}