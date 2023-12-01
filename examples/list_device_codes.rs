#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_device_codes()
        .cursor("your cursor")
        .location_id("your location id")
        .product_type("your product type")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}