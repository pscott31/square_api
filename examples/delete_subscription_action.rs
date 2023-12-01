#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let action_id = "your action id";
    let subscription_id = "your subscription id";
    let response = client
        .delete_subscription_action(action_id, subscription_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}