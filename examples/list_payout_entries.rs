#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let payout_id = "your payout id";
    let response = client
        .list_payout_entries(payout_id)
        .cursor("your cursor")
        .limit(1)
        .sort_order("your sort order")
        .await
        .unwrap();
    println!("{:#?}", response);
}