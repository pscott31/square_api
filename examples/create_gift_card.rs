#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let gift_card = GiftCard {
        balance_money: Some(Money {
            amount: Some(1),
            currency: Some("your currency".to_owned()),
        }),
        created_at: Some("your created at".to_owned()),
        customer_ids: Some(vec!["your customer ids".to_owned()]),
        gan: Some("your gan".to_owned()),
        gan_source: Some("your gan source".to_owned()),
        id: Some("your id".to_owned()),
        state: Some("your state".to_owned()),
        type_: "your type".to_owned(),
    };
    let idempotency_key = "your idempotency key";
    let location_id = "your location id";
    let response = client
        .create_gift_card(gift_card, idempotency_key, location_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}