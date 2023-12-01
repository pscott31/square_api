#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let location_id = "your location id";
    let shift_id = "your shift id";
    let response = client
        .retrieve_cash_drawer_shift(location_id, shift_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}