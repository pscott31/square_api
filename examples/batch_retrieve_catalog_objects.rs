#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let object_ids = &["your object ids"];
    let response = client
        .batch_retrieve_catalog_objects(object_ids)
        .catalog_version(1)
        .include_deleted_objects(true)
        .include_related_objects(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}