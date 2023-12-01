#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let custom_attribute_definition = CustomAttributeDefinition {
        created_at: Some("your created at".to_owned()),
        description: Some("your description".to_owned()),
        key: Some("your key".to_owned()),
        name: Some("your name".to_owned()),
        schema: Some(serde_json::json!({})),
        updated_at: Some("your updated at".to_owned()),
        version: Some(1),
        visibility: Some("your visibility".to_owned()),
    };
    let response = client
        .create_customer_custom_attribute_definition(custom_attribute_definition)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}