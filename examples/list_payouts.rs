#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_payouts()
        .begin_time("your begin time")
        .cursor("your cursor")
        .end_time("your end time")
        .limit(1)
        .location_id("your location id")
        .sort_order("your sort order")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}