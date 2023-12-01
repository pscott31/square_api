#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let v1_bank_account_id = "your v 1 bank account id";
    let response = client.get_bank_account_by_v1_id(v1_bank_account_id).await.unwrap();
    println!("{:#?}", response);
}