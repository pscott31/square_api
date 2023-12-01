#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_catalog()
        .catalog_version(1)
        .cursor("your cursor")
        .types("your types")
        .await
        .unwrap();
    println!("{:#?}", response);
}