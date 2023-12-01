#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_webhook_subscriptions()
        .cursor("your cursor")
        .include_disabled(true)
        .limit(1)
        .sort_order("your sort order")
        .await
        .unwrap();
    println!("{:#?}", response);
}