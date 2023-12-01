#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let customer_id = "your customer id";
    let response = client
        .list_customer_custom_attributes(customer_id)
        .cursor("your cursor")
        .limit(1)
        .with_definitions(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}