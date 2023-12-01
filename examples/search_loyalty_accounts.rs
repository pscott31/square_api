#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_loyalty_accounts()
        .cursor("your cursor")
        .limit(1)
        .query(SearchLoyaltyAccountsRequestLoyaltyAccountQuery {
            customer_ids: Some(vec!["your customer ids".to_owned()]),
            mappings: Some(
                vec![
                    LoyaltyAccountMapping { created_at : Some("your created at"
                    .to_owned()), id : Some("your id".to_owned()), phone_number :
                    Some("your phone number".to_owned()), type_ : Some("your type"
                    .to_owned()), value : Some("your value".to_owned()) }
                ],
            ),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}