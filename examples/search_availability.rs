#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let query = SearchAvailabilityQuery {
        filter: SearchAvailabilityFilter {
            booking_id: Some("your booking id".to_owned()),
            location_id: Some("your location id".to_owned()),
            segment_filters: Some(
                vec![
                    SegmentFilter { service_variation_id : "your service variation id"
                    .to_owned(), team_member_id_filter : Some(FilterValue { all :
                    Some(vec!["your all".to_owned()]), any : Some(vec!["your any"
                    .to_owned()]), none : Some(vec!["your none".to_owned()]) }) }
                ],
            ),
            start_at_range: TimeRange {
                end_at: Some("your end at".to_owned()),
                start_at: Some("your start at".to_owned()),
            },
        },
    };
    let response = client.search_availability(query).await.unwrap();
    println!("{:#?}", response);
}