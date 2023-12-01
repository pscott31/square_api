#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_terminal_checkouts()
        .cursor("your cursor")
        .limit(1)
        .query(TerminalCheckoutQuery {
            filter: Some(TerminalCheckoutQueryFilter {
                created_at: Some(TimeRange {
                    end_at: Some("your end at".to_owned()),
                    start_at: Some("your start at".to_owned()),
                }),
                device_id: Some("your device id".to_owned()),
                status: Some("your status".to_owned()),
            }),
            sort: Some(TerminalCheckoutQuerySort {
                sort_order: Some("your sort order".to_owned()),
            }),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}