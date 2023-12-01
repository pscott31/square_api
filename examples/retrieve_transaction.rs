#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let location_id = "your location id";
    let transaction_id = "your transaction id";
    let response = client
        .retrieve_transaction(location_id, transaction_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}