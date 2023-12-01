#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let customer_id = "your customer id";
    let location_id = "your location id";
    let plan_id = "your plan id";
    let response = client
        .create_subscription(customer_id, location_id, plan_id)
        .canceled_date("your canceled date")
        .card_id("your card id")
        .idempotency_key("your idempotency key")
        .price_override_money(Money {
            amount: Some(1),
            currency: Some("your currency".to_owned()),
        })
        .source(SubscriptionSource {
            name: Some("your name".to_owned()),
        })
        .start_date("your start date")
        .tax_percentage("your tax percentage")
        .timezone("your timezone")
        .await
        .unwrap();
    println!("{:#?}", response);
}