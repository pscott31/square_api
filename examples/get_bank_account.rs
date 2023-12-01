#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let bank_account_id = "your bank account id";
    let response = client.get_bank_account(bank_account_id).await.unwrap();
    println!("{:#?}", response);
}