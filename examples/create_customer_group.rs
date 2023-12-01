#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let group = CustomerGroup {
        created_at: Some("your created at".to_owned()),
        id: Some("your id".to_owned()),
        name: "your name".to_owned(),
        updated_at: Some("your updated at".to_owned()),
    };
    let response = client
        .create_customer_group(group)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}