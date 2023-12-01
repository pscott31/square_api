#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_team_members()
        .cursor("your cursor")
        .limit(1)
        .query(SearchTeamMembersQuery {
            filter: Some(SearchTeamMembersFilter {
                is_owner: Some(true),
                location_ids: Some(vec!["your location ids".to_owned()]),
                status: Some("your status".to_owned()),
            }),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}