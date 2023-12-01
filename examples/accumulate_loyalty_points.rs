#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
use square_api::request::AccumulateLoyaltyPointsRequired;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let args = AccumulateLoyaltyPointsRequired {
        account_id: "your account id",
        accumulate_points: LoyaltyEventAccumulatePoints {
            loyalty_program_id: Some("your loyalty program id".to_owned()),
            order_id: Some("your order id".to_owned()),
            points: Some(1),
        },
        idempotency_key: "your idempotency key",
        location_id: "your location id",
    };
    let response = client.accumulate_loyalty_points(args).await.unwrap();
    println!("{:#?}", response);
}