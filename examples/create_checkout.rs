#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let idempotency_key = "your idempotency key";
    let location_id = "your location id";
    let order = CreateOrderRequest {
        idempotency_key: Some("your idempotency key".to_owned()),
        order: None,
    };
    let response = client
        .create_checkout(idempotency_key, location_id, order)
        .additional_recipients(
            vec![
                ChargeRequestAdditionalRecipient { amount_money : Money { amount :
                Some(1), currency : Some("your currency".to_owned()) }, description :
                "your description".to_owned(), location_id : "your location id"
                .to_owned() }
            ],
        )
        .ask_for_shipping_address(true)
        .merchant_support_email("your merchant support email")
        .note("your note")
        .pre_populate_buyer_email("your pre populate buyer email")
        .pre_populate_shipping_address(Address {
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
        .redirect_url("your redirect url")
        .await
        .unwrap();
    println!("{:#?}", response);
}