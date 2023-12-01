#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let reward_id = "your reward id";
    let response = client.retrieve_loyalty_reward(reward_id).await.unwrap();
    println!("{:#?}", response);
}