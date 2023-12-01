#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let action_id = "your action id";
    let response = client.cancel_terminal_action(action_id).await.unwrap();
    println!("{:#?}", response);
}