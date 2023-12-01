#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let item_ids = &["your item ids"];
    let response = client
        .update_item_taxes(item_ids)
        .taxes_to_disable(&["your taxes to disable"])
        .taxes_to_enable(&["your taxes to enable"])
        .await
        .unwrap();
    println!("{:#?}", response);
}