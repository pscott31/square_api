#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let location_id = "your location id";
    let response = client
        .list_transactions(location_id)
        .begin_time("your begin time")
        .cursor("your cursor")
        .end_time("your end time")
        .sort_order("your sort order")
        .await
        .unwrap();
    println!("{:#?}", response);
}