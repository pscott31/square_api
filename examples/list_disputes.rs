#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_disputes()
        .cursor("your cursor")
        .location_id("your location id")
        .states("your states")
        .await
        .unwrap();
    println!("{:#?}", response);
}