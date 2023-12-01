#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_payments()
        .begin_time("your begin time")
        .card_brand("your card brand")
        .cursor("your cursor")
        .end_time("your end time")
        .last4("your last 4")
        .limit(1)
        .location_id("your location id")
        .sort_order("your sort order")
        .total(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}