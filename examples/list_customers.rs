#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_customers()
        .cursor("your cursor")
        .limit(1)
        .sort_field("your sort field")
        .sort_order("your sort order")
        .await
        .unwrap();
    println!("{:#?}", response);
}