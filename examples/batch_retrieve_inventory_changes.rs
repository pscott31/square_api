#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .batch_retrieve_inventory_changes()
        .catalog_object_ids(&["your catalog object ids"])
        .cursor("your cursor")
        .limit(1)
        .location_ids(&["your location ids"])
        .states(&["your states"])
        .statuses(&["your statuses"])
        .types(&["your types"])
        .updated_after("your updated after")
        .updated_before("your updated before")
        .await
        .unwrap();
    println!("{:#?}", response);
}