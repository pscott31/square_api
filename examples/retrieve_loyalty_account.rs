#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let account_id = "your account id";
    let response = client.retrieve_loyalty_account(account_id).await.unwrap();
    println!("{:#?}", response);
}