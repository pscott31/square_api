#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let vendor_id = "your vendor id";
    let response = client.retrieve_vendor(vendor_id).await.unwrap();
    println!("{:#?}", response);
}