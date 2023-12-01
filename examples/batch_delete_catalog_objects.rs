#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .batch_delete_catalog_objects()
        .object_ids(&["your object ids"])
        .await
        .unwrap();
    println!("{:#?}", response);
}