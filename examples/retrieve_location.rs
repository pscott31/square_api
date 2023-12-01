#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let location_id = "your location id";
    let response = client.retrieve_location(location_id).await.unwrap();
    println!("{:#?}", response);
}