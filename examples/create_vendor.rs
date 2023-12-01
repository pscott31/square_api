#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let idempotency_key = "your idempotency key";
    let response = client
        .create_vendor(idempotency_key)
        .vendor(Vendor {
            account_number: Some("your account number".to_owned()),
            address: Some(Address {
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
            contacts: Some(
                vec![
                    VendorContact { email_address : Some("your email address"
                    .to_owned()), id : Some("your id".to_owned()), name :
                    Some("your name".to_owned()), ordinal : 1, phone_number :
                    Some("your phone number".to_owned()), removed : Some(true) }
                ],
            ),
            created_at: Some("your created at".to_owned()),
            id: Some("your id".to_owned()),
            name: Some("your name".to_owned()),
            note: Some("your note".to_owned()),
            status: Some("your status".to_owned()),
            updated_at: Some("your updated at".to_owned()),
            version: Some(1),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}