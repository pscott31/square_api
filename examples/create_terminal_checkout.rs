#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let checkout = TerminalCheckout {
        amount_money: Money {
            amount: Some(1),
            currency: Some("your currency".to_owned()),
        },
        app_fee_money: Some(Money {
            amount: Some(1),
            currency: Some("your currency".to_owned()),
        }),
        app_id: Some("your app id".to_owned()),
        cancel_reason: Some("your cancel reason".to_owned()),
        created_at: Some("your created at".to_owned()),
        customer_id: Some("your customer id".to_owned()),
        deadline_duration: Some("your deadline duration".to_owned()),
        device_options: DeviceCheckoutOptions {
            collect_signature: Some(true),
            device_id: "your device id".to_owned(),
            show_itemized_cart: Some(true),
            skip_receipt_screen: Some(true),
            tip_settings: Some(TipSettings {
                allow_tipping: Some(true),
                custom_tip_field: Some(true),
                separate_tip_screen: Some(true),
                smart_tipping: Some(true),
                tip_percentages: Some(vec![1]),
            }),
        },
        id: Some("your id".to_owned()),
        location_id: Some("your location id".to_owned()),
        note: Some("your note".to_owned()),
        order_id: Some("your order id".to_owned()),
        payment_ids: Some(vec!["your payment ids".to_owned()]),
        payment_options: Some(PaymentOptions {
            accept_partial_authorization: Some(true),
            autocomplete: Some(true),
            delay_duration: Some("your delay duration".to_owned()),
        }),
        payment_type: Some("your payment type".to_owned()),
        reference_id: Some("your reference id".to_owned()),
        status: Some("your status".to_owned()),
        updated_at: Some("your updated at".to_owned()),
    };
    let idempotency_key = "your idempotency key";
    let response = client
        .create_terminal_checkout(checkout, idempotency_key)
        .await
        .unwrap();
    println!("{:#?}", response);
}