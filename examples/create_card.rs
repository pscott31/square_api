#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let card = Card {
        billing_address: Some(Address {
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
        }),
        bin: Some("your bin".to_owned()),
        card_brand: Some("your card brand".to_owned()),
        card_co_brand: Some("your card co brand".to_owned()),
        card_type: Some("your card type".to_owned()),
        cardholder_name: Some("your cardholder name".to_owned()),
        customer_id: Some("your customer id".to_owned()),
        enabled: Some(true),
        exp_month: Some(1),
        exp_year: Some(1),
        fingerprint: Some("your fingerprint".to_owned()),
        id: Some("your id".to_owned()),
        last4: Some("your last 4".to_owned()),
        merchant_id: Some("your merchant id".to_owned()),
        prepaid_type: Some("your prepaid type".to_owned()),
        reference_id: Some("your reference id".to_owned()),
        version: Some(1),
    };
    let idempotency_key = "your idempotency key";
    let source_id = "your source id";
    let response = client
        .create_card(card, idempotency_key, source_id)
        .verification_token("your verification token")
        .await
        .unwrap();
    println!("{:#?}", response);
}