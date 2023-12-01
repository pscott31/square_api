#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let segment_id = "your segment id";
    let response = client.retrieve_customer_segment(segment_id).await.unwrap();
    println!("{:#?}", response);
}