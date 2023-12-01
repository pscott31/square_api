#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let amount_money = Money {
        amount: Some(1),
        currency: Some("your currency".to_owned()),
    };
    let idempotency_key = "your idempotency key";
    let response = client
        .refund_payment(amount_money, idempotency_key)
        .app_fee_money(Money {
            amount: Some(1),
            currency: Some("your currency".to_owned()),
        })
        .payment_id("your payment id")
        .payment_version_token("your payment version token")
        .reason("your reason")
        .team_member_id("your team member id")
        .await
        .unwrap();
    println!("{:#?}", response);
}