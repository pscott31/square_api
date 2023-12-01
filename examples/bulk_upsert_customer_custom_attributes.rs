#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let values = serde_json::json!({});
    let response = client.bulk_upsert_customer_custom_attributes(values).await.unwrap();
    println!("{:#?}", response);
}