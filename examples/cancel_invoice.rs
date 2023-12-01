#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let invoice_id = "your invoice id";
    let version = 1;
    let response = client.cancel_invoice(invoice_id, version).await.unwrap();
    println!("{:#?}", response);
}