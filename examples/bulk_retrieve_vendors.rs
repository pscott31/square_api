#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .bulk_retrieve_vendors()
        .vendor_ids(&["your vendor ids"])
        .await
        .unwrap();
    println!("{:#?}", response);
}