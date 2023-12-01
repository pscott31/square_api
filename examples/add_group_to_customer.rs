#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let customer_id = "your customer id";
    let group_id = "your group id";
    let response = client.add_group_to_customer(customer_id, group_id).await.unwrap();
    println!("{:#?}", response);
}