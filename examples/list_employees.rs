#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_employees()
        .cursor("your cursor")
        .limit(1)
        .location_id("your location id")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}