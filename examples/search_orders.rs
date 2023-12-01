#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_orders()
        .cursor("your cursor")
        .limit(1)
        .location_ids(&["your location ids"])
        .query(SearchOrdersQuery {
            filter: Some(SearchOrdersFilter {
                customer_filter: Some(SearchOrdersCustomerFilter {
                    customer_ids: Some(vec!["your customer ids".to_owned()]),
                }),
                date_time_filter: Some(SearchOrdersDateTimeFilter {
                    closed_at: Some(TimeRange {
                        end_at: Some("your end at".to_owned()),
                        start_at: Some("your start at".to_owned()),
                    }),
                    created_at: Some(TimeRange {
                        end_at: Some("your end at".to_owned()),
                        start_at: Some("your start at".to_owned()),
                    }),
                    updated_at: Some(TimeRange {
                        end_at: Some("your end at".to_owned()),
                        start_at: Some("your start at".to_owned()),
                    }),
                }),
                fulfillment_filter: Some(SearchOrdersFulfillmentFilter {
                    fulfillment_states: Some(vec!["your fulfillment states".to_owned()]),
                    fulfillment_types: Some(vec!["your fulfillment types".to_owned()]),
                }),
                source_filter: Some(SearchOrdersSourceFilter {
                    source_names: Some(vec!["your source names".to_owned()]),
                }),
                state_filter: Some(SearchOrdersStateFilter {
                    states: vec!["your states".to_owned()],
                }),
            }),
            sort: Some(SearchOrdersSort {
                sort_field: "your sort field".to_owned(),
                sort_order: Some("your sort order".to_owned()),
            }),
        })
        .return_entries(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}