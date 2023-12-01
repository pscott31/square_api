#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let customer_id = "your customer id";
    let gift_card_id = "your gift card id";
    let response = client
        .unlink_customer_from_gift_card(customer_id, gift_card_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}