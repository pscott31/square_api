#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_workweek_configs()
        .cursor("your cursor")
        .limit(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}