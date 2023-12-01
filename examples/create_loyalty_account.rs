#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let idempotency_key = "your idempotency key";
    let loyalty_account = LoyaltyAccount {
        balance: Some(1),
        created_at: Some("your created at".to_owned()),
        customer_id: Some("your customer id".to_owned()),
        enrolled_at: Some("your enrolled at".to_owned()),
        expiring_point_deadlines: Some(
            vec![
                LoyaltyAccountExpiringPointDeadline { expires_at : "your expires at"
                .to_owned(), points : 1 }
            ],
        ),
        id: Some("your id".to_owned()),
        lifetime_points: Some(1),
        mapping: Some(LoyaltyAccountMapping {
            created_at: Some("your created at".to_owned()),
            id: Some("your id".to_owned()),
            phone_number: Some("your phone number".to_owned()),
            type_: Some("your type".to_owned()),
            value: Some("your value".to_owned()),
        }),
        mappings: Some(
            vec![
                LoyaltyAccountMapping { created_at : Some("your created at".to_owned()),
                id : Some("your id".to_owned()), phone_number : Some("your phone number"
                .to_owned()), type_ : Some("your type".to_owned()), value :
                Some("your value".to_owned()) }
            ],
        ),
        program_id: "your program id".to_owned(),
        updated_at: Some("your updated at".to_owned()),
    };
    let response = client
        .create_loyalty_account(idempotency_key, loyalty_account)
        .await
        .unwrap();
    println!("{:#?}", response);
}