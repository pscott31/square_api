#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let team_member_id = "your team member id";
    let response = client.retrieve_wage_setting(team_member_id).await.unwrap();
    println!("{:#?}", response);
}