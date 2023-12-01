#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .search_shifts()
        .cursor("your cursor")
        .limit(1)
        .query(ShiftQuery {
            filter: Some(ShiftFilter {
                employee_ids: Some(vec!["your employee ids".to_owned()]),
                end: Some(TimeRange {
                    end_at: Some("your end at".to_owned()),
                    start_at: Some("your start at".to_owned()),
                }),
                location_ids: Some(vec!["your location ids".to_owned()]),
                start: Some(TimeRange {
                    end_at: Some("your end at".to_owned()),
                    start_at: Some("your start at".to_owned()),
                }),
                status: Some("your status".to_owned()),
                team_member_ids: Some(vec!["your team member ids".to_owned()]),
                workday: Some(ShiftWorkday {
                    date_range: Some(DateRange {
                        end_date: Some("your end date".to_owned()),
                        start_date: Some("your start date".to_owned()),
                    }),
                    default_timezone: Some("your default timezone".to_owned()),
                    match_shifts_by: Some("your match shifts by".to_owned()),
                }),
            }),
            sort: Some(ShiftSort {
                field: Some("your field".to_owned()),
                order: Some("your order".to_owned()),
            }),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}