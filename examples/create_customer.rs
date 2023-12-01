#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .create_customer()
        .address(Address {
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
        .birthday("your birthday")
        .company_name("your company name")
        .email_address("your email address")
        .family_name("your family name")
        .given_name("your given name")
        .idempotency_key("your idempotency key")
        .nickname("your nickname")
        .note("your note")
        .phone_number("your phone number")
        .reference_id("your reference id")
        .tax_ids(CustomerTaxIds {
            eu_vat: Some("your eu vat".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}