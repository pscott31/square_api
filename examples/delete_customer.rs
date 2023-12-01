#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let customer_id = "your customer id";
    let response = client.delete_customer(customer_id).version(1).await.unwrap();
    println!("{:#?}", response);
}