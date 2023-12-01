#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_cards()
        .cursor("your cursor")
        .customer_id("your customer id")
        .include_disabled(true)
        .reference_id("your reference id")
        .sort_order("your sort order")
        .await
        .unwrap();
    println!("{:#?}", response);
}