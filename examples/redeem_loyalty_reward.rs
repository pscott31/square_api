#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let idempotency_key = "your idempotency key";
    let location_id = "your location id";
    let reward_id = "your reward id";
    let response = client
        .redeem_loyalty_reward(idempotency_key, location_id, reward_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}