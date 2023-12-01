#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let shift = Shift {
        breaks: Some(
            vec![
                Break { break_type_id : "your break type id".to_owned(), end_at :
                Some("your end at".to_owned()), expected_duration :
                "your expected duration".to_owned(), id : Some("your id".to_owned()),
                is_paid : true, name : "your name".to_owned(), start_at : "your start at"
                .to_owned() }
            ],
        ),
        created_at: Some("your created at".to_owned()),
        employee_id: Some("your employee id".to_owned()),
        end_at: Some("your end at".to_owned()),
        id: Some("your id".to_owned()),
        location_id: Some("your location id".to_owned()),
        start_at: "your start at".to_owned(),
        status: Some("your status".to_owned()),
        team_member_id: Some("your team member id".to_owned()),
        timezone: Some("your timezone".to_owned()),
        updated_at: Some("your updated at".to_owned()),
        version: Some(1),
        wage: Some(ShiftWage {
            hourly_rate: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            title: Some("your title".to_owned()),
        }),
    };
    let response = client
        .create_shift(shift)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}