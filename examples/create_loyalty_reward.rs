#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let idempotency_key = "your idempotency key";
    let reward = LoyaltyReward {
        created_at: Some("your created at".to_owned()),
        id: Some("your id".to_owned()),
        loyalty_account_id: "your loyalty account id".to_owned(),
        order_id: Some("your order id".to_owned()),
        points: Some(1),
        redeemed_at: Some("your redeemed at".to_owned()),
        reward_tier_id: "your reward tier id".to_owned(),
        status: Some("your status".to_owned()),
        updated_at: Some("your updated at".to_owned()),
    };
    let response = client.create_loyalty_reward(idempotency_key, reward).await.unwrap();
    println!("{:#?}", response);
}