#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let idempotency_key = "your idempotency key";
    let response = client
        .create_terminal_refund(idempotency_key)
        .refund(TerminalRefund {
            amount_money: Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            },
            app_id: Some("your app id".to_owned()),
            cancel_reason: Some("your cancel reason".to_owned()),
            created_at: Some("your created at".to_owned()),
            deadline_duration: Some("your deadline duration".to_owned()),
            device_id: "your device id".to_owned(),
            id: Some("your id".to_owned()),
            location_id: Some("your location id".to_owned()),
            order_id: Some("your order id".to_owned()),
            payment_id: "your payment id".to_owned(),
            reason: "your reason".to_owned(),
            refund_id: Some("your refund id".to_owned()),
            status: Some("your status".to_owned()),
            updated_at: Some("your updated at".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}