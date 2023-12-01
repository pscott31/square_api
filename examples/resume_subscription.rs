#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let subscription_id = "your subscription id";
    let response = client
        .resume_subscription(subscription_id)
        .resume_change_timing("your resume change timing")
        .resume_effective_date("your resume effective date")
        .await
        .unwrap();
    println!("{:#?}", response);
}