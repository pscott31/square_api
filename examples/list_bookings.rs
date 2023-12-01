#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_bookings()
        .cursor("your cursor")
        .limit(1)
        .location_id("your location id")
        .start_at_max("your start at max")
        .start_at_min("your start at min")
        .team_member_id("your team member id")
        .await
        .unwrap();
    println!("{:#?}", response);
}