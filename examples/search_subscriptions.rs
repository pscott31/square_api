#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_subscriptions()
        .cursor("your cursor")
        .include(&["your include"])
        .limit(1)
        .query(SearchSubscriptionsQuery {
            filter: Some(SearchSubscriptionsFilter {
                customer_ids: Some(vec!["your customer ids".to_owned()]),
                location_ids: Some(vec!["your location ids".to_owned()]),
                source_names: Some(vec!["your source names".to_owned()]),
            }),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}