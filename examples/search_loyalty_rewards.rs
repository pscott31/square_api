#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_loyalty_rewards()
        .cursor("your cursor")
        .limit(1)
        .query(SearchLoyaltyRewardsRequestLoyaltyRewardQuery {
            loyalty_account_id: "your loyalty account id".to_owned(),
            status: Some("your status".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}