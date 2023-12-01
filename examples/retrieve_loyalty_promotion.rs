#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let program_id = "your program id";
    let promotion_id = "your promotion id";
    let response = client
        .retrieve_loyalty_promotion(program_id, promotion_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}