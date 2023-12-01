#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .list_team_member_wages()
        .cursor("your cursor")
        .limit(1)
        .team_member_id("your team member id")
        .await
        .unwrap();
    println!("{:#?}", response);
}