#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_webhook_event_types()
        .api_version("your api version")
        .await
        .unwrap();
    println!("{:#?}", response);
}