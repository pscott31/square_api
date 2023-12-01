#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let object_id = "your object id";
    let response = client
        .retrieve_catalog_object(object_id)
        .catalog_version(1)
        .include_related_objects(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}