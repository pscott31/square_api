#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_gift_cards()
        .cursor("your cursor")
        .customer_id("your customer id")
        .limit(1)
        .state("your state")
        .type_("your type")
        .await
        .unwrap();
    println!("{:#?}", response);
}