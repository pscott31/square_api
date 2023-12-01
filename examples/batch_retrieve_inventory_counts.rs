#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .batch_retrieve_inventory_counts()
        .catalog_object_ids(&["your catalog object ids"])
        .cursor("your cursor")
        .limit(1)
        .location_ids(&["your location ids"])
        .states(&["your states"])
        .updated_after("your updated after")
        .await
        .unwrap();
    println!("{:#?}", response);
}