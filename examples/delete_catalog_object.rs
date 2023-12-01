#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let object_id = "your object id";
    let response = client.delete_catalog_object(object_id).await.unwrap();
    println!("{:#?}", response);
}