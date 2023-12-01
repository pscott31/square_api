#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let action = TerminalAction {
        app_id: Some("your app id".to_owned()),
        cancel_reason: Some("your cancel reason".to_owned()),
        created_at: Some("your created at".to_owned()),
        deadline_duration: Some("your deadline duration".to_owned()),
        device_id: Some("your device id".to_owned()),
        device_metadata: Some(DeviceMetadata {
            app_version: Some("your app version".to_owned()),
            battery_percentage: Some("your battery percentage".to_owned()),
            charging_state: Some("your charging state".to_owned()),
            ip_address: Some("your ip address".to_owned()),
            location_id: Some("your location id".to_owned()),
            merchant_id: Some("your merchant id".to_owned()),
            network_connection_type: Some("your network connection type".to_owned()),
            os_version: Some("your os version".to_owned()),
            payment_region: Some("your payment region".to_owned()),
            serial_number: Some("your serial number".to_owned()),
            wifi_network_name: Some("your wifi network name".to_owned()),
            wifi_network_strength: Some("your wifi network strength".to_owned()),
        }),
        id: Some("your id".to_owned()),
        save_card_options: Some(SaveCardOptions {
            card_id: Some("your card id".to_owned()),
            customer_id: "your customer id".to_owned(),
            reference_id: Some("your reference id".to_owned()),
        }),
        status: Some("your status".to_owned()),
        type_: Some("your type".to_owned()),
        updated_at: Some("your updated at".to_owned()),
    };
    let idempotency_key = "your idempotency key";
    let response = client.create_terminal_action(action, idempotency_key).await.unwrap();
    println!("{:#?}", response);
}