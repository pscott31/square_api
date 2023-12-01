#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let invoice_id = "your invoice id";
    let version = 1;
    let response = client
        .publish_invoice(invoice_id, version)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}