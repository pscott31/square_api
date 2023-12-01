#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let team_members = serde_json::json!({});
    let response = client.bulk_update_team_members(team_members).await.unwrap();
    println!("{:#?}", response);
}