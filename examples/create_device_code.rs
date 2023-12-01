#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let device_code = DeviceCode {
        code: Some("your code".to_owned()),
        created_at: Some("your created at".to_owned()),
        device_id: Some("your device id".to_owned()),
        id: Some("your id".to_owned()),
        location_id: Some("your location id".to_owned()),
        name: Some("your name".to_owned()),
        pair_by: Some("your pair by".to_owned()),
        paired_at: Some("your paired at".to_owned()),
        product_type: "your product type".to_owned(),
        status: Some("your status".to_owned()),
        status_changed_at: Some("your status changed at".to_owned()),
    };
    let idempotency_key = "your idempotency key";
    let response = client
        .create_device_code(device_code, idempotency_key)
        .await
        .unwrap();
    println!("{:#?}", response);
}