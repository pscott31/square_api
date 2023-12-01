#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let booking = Booking {
        all_day: Some(true),
        appointment_segments: Some(
            vec![
                AppointmentSegment { any_team_member : Some(true), duration_minutes :
                Some(1), intermission_minutes : Some(1), resource_ids :
                Some(vec!["your resource ids".to_owned()]), service_variation_id :
                Some("your service variation id".to_owned()), service_variation_version :
                Some(1), team_member_id : "your team member id".to_owned() }
            ],
        ),
        created_at: Some("your created at".to_owned()),
        creator_details: Some(BookingCreatorDetails {
            creator_type: Some("your creator type".to_owned()),
            customer_id: Some("your customer id".to_owned()),
            team_member_id: Some("your team member id".to_owned()),
        }),
        customer_id: Some("your customer id".to_owned()),
        customer_note: Some("your customer note".to_owned()),
        id: Some("your id".to_owned()),
        location_id: Some("your location id".to_owned()),
        location_type: Some("your location type".to_owned()),
        seller_note: Some("your seller note".to_owned()),
        source: Some("your source".to_owned()),
        start_at: Some("your start at".to_owned()),
        status: Some("your status".to_owned()),
        transition_time_minutes: Some(1),
        updated_at: Some("your updated at".to_owned()),
        version: Some(1),
    };
    let booking_id = "your booking id";
    let response = client
        .update_booking(booking, booking_id)
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}