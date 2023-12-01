#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let subscription = WebhookSubscription {
        api_version: Some("your api version".to_owned()),
        created_at: Some("your created at".to_owned()),
        enabled: Some(true),
        event_types: Some(vec!["your event types".to_owned()]),
        id: Some("your id".to_owned()),
        name: Some("your name".to_owned()),
        notification_url: Some("your notification url".to_owned()),
        signature_key: Some("your signature key".to_owned()),
        updated_at: Some("your updated at".to_owned()),
    };
    let response = client
        .create_webhook_subscription(subscription)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}