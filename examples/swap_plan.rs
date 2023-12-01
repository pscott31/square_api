#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let new_plan_id = "your new plan id";
    let subscription_id = "your subscription id";
    let response = client.swap_plan(new_plan_id, subscription_id).await.unwrap();
    println!("{:#?}", response);
}