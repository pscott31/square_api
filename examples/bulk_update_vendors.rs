#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let vendors = serde_json::json!({});
    let response = client.bulk_update_vendors(vendors).await.unwrap();
    println!("{:#?}", response);
}