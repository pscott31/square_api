#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let program_id = "your program id";
    let response = client
        .list_loyalty_promotions(program_id)
        .cursor("your cursor")
        .limit(1)
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}