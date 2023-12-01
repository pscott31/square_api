#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let payment_id = "your payment id";
    let response = client.get_payment(payment_id).await.unwrap();
    println!("{:#?}", response);
}