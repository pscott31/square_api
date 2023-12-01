#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let id = "your id";
    let response = client.get_team_member_wage(id).await.unwrap();
    println!("{:#?}", response);
}