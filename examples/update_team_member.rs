#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let team_member_id = "your team member id";
    let response = client
        .update_team_member(team_member_id)
        .team_member(TeamMember {
            assigned_locations: Some(TeamMemberAssignedLocations {
                assignment_type: Some("your assignment type".to_owned()),
                location_ids: Some(vec!["your location ids".to_owned()]),
            }),
            created_at: Some("your created at".to_owned()),
            email_address: Some("your email address".to_owned()),
            family_name: Some("your family name".to_owned()),
            given_name: Some("your given name".to_owned()),
            id: Some("your id".to_owned()),
            is_owner: Some(true),
            phone_number: Some("your phone number".to_owned()),
            reference_id: Some("your reference id".to_owned()),
            status: Some("your status".to_owned()),
            updated_at: Some("your updated at".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}