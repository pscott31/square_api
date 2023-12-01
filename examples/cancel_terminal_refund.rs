#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let terminal_refund_id = "your terminal refund id";
    let response = client.cancel_terminal_refund(terminal_refund_id).await.unwrap();
    println!("{:#?}", response);
}