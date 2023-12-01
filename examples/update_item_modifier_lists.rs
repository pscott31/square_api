#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let item_ids = &["your item ids"];
    let response = client
        .update_item_modifier_lists(item_ids)
        .modifier_lists_to_disable(&["your modifier lists to disable"])
        .modifier_lists_to_enable(&["your modifier lists to enable"])
        .await
        .unwrap();
    println!("{:#?}", response);
}