#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let break_type = BreakType {
        break_name: "your break name".to_owned(),
        created_at: Some("your created at".to_owned()),
        expected_duration: "your expected duration".to_owned(),
        id: Some("your id".to_owned()),
        is_paid: true,
        location_id: "your location id".to_owned(),
        updated_at: Some("your updated at".to_owned()),
        version: Some(1),
    };
    let response = client
        .create_break_type(break_type)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}