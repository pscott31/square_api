#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let customer_id = "your customer id";
    let key = "your key";
    let response = client
        .retrieve_customer_custom_attribute(customer_id, key)
        .version(1)
        .with_definition(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}