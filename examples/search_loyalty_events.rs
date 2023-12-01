#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_loyalty_events()
        .cursor("your cursor")
        .limit(1)
        .query(LoyaltyEventQuery {
            filter: Some(LoyaltyEventFilter {
                date_time_filter: Some(LoyaltyEventDateTimeFilter {
                    created_at: TimeRange {
                        end_at: Some("your end at".to_owned()),
                        start_at: Some("your start at".to_owned()),
                    },
                }),
                location_filter: Some(LoyaltyEventLocationFilter {
                    location_ids: vec!["your location ids".to_owned()],
                }),
                loyalty_account_filter: Some(LoyaltyEventLoyaltyAccountFilter {
                    loyalty_account_id: "your loyalty account id".to_owned(),
                }),
                order_filter: Some(LoyaltyEventOrderFilter {
                    order_id: "your order id".to_owned(),
                }),
                type_filter: Some(LoyaltyEventTypeFilter {
                    types: vec!["your types".to_owned()],
                }),
            }),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}