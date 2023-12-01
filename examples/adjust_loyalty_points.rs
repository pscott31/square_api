#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let account_id = "your account id";
    let adjust_points = LoyaltyEventAdjustPoints {
        loyalty_program_id: Some("your loyalty program id".to_owned()),
        points: 1,
        reason: Some("your reason".to_owned()),
    };
    let idempotency_key = "your idempotency key";
    let response = client
        .adjust_loyalty_points(account_id, adjust_points, idempotency_key)
        .allow_negative_balance(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}