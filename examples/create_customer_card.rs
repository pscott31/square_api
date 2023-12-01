#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let card_nonce = "your card nonce";
    let customer_id = "your customer id";
    let response = client
        .create_customer_card(card_nonce, customer_id)
        .billing_address(Address {
            address_line1: Some("your address line 1".to_owned()),
            address_line2: Some("your address line 2".to_owned()),
            address_line3: Some("your address line 3".to_owned()),
            administrative_district_level1: Some(
                "your administrative district level 1".to_owned(),
            ),
            administrative_district_level2: Some(
                "your administrative district level 2".to_owned(),
            ),
            administrative_district_level3: Some(
                "your administrative district level 3".to_owned(),
            ),
            country: Some("your country".to_owned()),
            first_name: Some("your first name".to_owned()),
            last_name: Some("your last name".to_owned()),
            locality: Some("your locality".to_owned()),
            postal_code: Some("your postal code".to_owned()),
            sublocality: Some("your sublocality".to_owned()),
            sublocality2: Some("your sublocality 2".to_owned()),
            sublocality3: Some("your sublocality 3".to_owned()),
        })
        .cardholder_name("your cardholder name")
        .verification_token("your verification token")
        .await
        .unwrap();
    println!("{:#?}", response);
}