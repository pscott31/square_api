#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let team_member_id = "your team member id";
    let wage_setting = WageSetting {
        created_at: Some("your created at".to_owned()),
        is_overtime_exempt: Some(true),
        job_assignments: Some(
            vec![
                JobAssignment { annual_rate : Some(Money { amount : Some(1), currency :
                Some("your currency".to_owned()) }), hourly_rate : Some(Money { amount :
                Some(1), currency : Some("your currency".to_owned()) }), job_title :
                "your job title".to_owned(), pay_type : "your pay type".to_owned(),
                weekly_hours : Some(1) }
            ],
        ),
        team_member_id: Some("your team member id".to_owned()),
        updated_at: Some("your updated at".to_owned()),
        version: Some(1),
    };
    let response = client
        .update_wage_setting(team_member_id, wage_setting)
        .await
        .unwrap();
    println!("{:#?}", response);
}