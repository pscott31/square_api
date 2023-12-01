#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let location_id = "your location id";
    let shift_id = "your shift id";
    let response = client
        .list_cash_drawer_shift_events(location_id, shift_id)
        .cursor("your cursor")
        .limit(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}