#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let key = "your key";
    let response = client
        .delete_customer_custom_attribute_definition(key)
        .await
        .unwrap();
    println!("{:#?}", response);
}