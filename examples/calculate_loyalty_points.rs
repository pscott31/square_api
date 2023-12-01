#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let program_id = "your program id";
    let response = client
        .calculate_loyalty_points(program_id)
        .loyalty_account_id("your loyalty account id")
        .order_id("your order id")
        .transaction_amount_money(Money {
            amount: Some(1),
            currency: Some("your currency".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}