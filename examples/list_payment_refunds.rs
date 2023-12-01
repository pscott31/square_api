#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_payment_refunds()
        .begin_time("your begin time")
        .cursor("your cursor")
        .end_time("your end time")
        .limit(1)
        .location_id("your location id")
        .sort_order("your sort order")
        .source_type("your source type")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}