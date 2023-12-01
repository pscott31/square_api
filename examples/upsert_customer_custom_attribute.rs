#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let custom_attribute = CustomAttribute {
        created_at: Some("your created at".to_owned()),
        definition: Some(CustomAttributeDefinition {
            created_at: Some("your created at".to_owned()),
            description: Some("your description".to_owned()),
            key: Some("your key".to_owned()),
            name: Some("your name".to_owned()),
            schema: Some(serde_json::json!({})),
            updated_at: Some("your updated at".to_owned()),
            version: Some(1),
            visibility: Some("your visibility".to_owned()),
        }),
        key: Some("your key".to_owned()),
        updated_at: Some("your updated at".to_owned()),
        value: Some(serde_json::json!({})),
        version: Some(1),
        visibility: Some("your visibility".to_owned()),
    };
    let customer_id = "your customer id";
    let key = "your key";
    let response = client
        .upsert_customer_custom_attribute(custom_attribute, customer_id, key)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}