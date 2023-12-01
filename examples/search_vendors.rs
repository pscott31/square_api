#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_vendors()
        .cursor("your cursor")
        .filter(SearchVendorsRequestFilter {
            name: Some(vec!["your name".to_owned()]),
            status: Some(vec!["your status".to_owned()]),
        })
        .sort(SearchVendorsRequestSort {
            field: Some("your field".to_owned()),
            order: Some("your order".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}