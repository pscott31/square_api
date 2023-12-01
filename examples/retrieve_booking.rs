#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let booking_id = "your booking id";
    let response = client.retrieve_booking(booking_id).await.unwrap();
    println!("{:#?}", response);
}