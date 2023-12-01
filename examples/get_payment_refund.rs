#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let refund_id = "your refund id";
    let response = client.get_payment_refund(refund_id).await.unwrap();
    println!("{:#?}", response);
}