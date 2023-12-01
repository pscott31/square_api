#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let invoice_id = "your invoice id";
    let response = client.delete_invoice(invoice_id).version(1).await.unwrap();
    println!("{:#?}", response);
}