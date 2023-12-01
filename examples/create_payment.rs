#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let amount_money = Money {
        amount: Some(1),
        currency: Some("your currency".to_owned()),
    };
    let idempotency_key = "your idempotency key";
    let source_id = "your source id";
    let response = client
        .create_payment(amount_money, idempotency_key, source_id)
        .accept_partial_authorization(true)
        .app_fee_money(Money {
            amount: Some(1),
            currency: Some("your currency".to_owned()),
        })
        .autocomplete(true)
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
        .buyer_email_address("your buyer email address")
        .cash_details(CashPaymentDetails {
            buyer_supplied_money: Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            },
            change_back_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
        })
        .customer_id("your customer id")
        .delay_action("your delay action")
        .delay_duration("your delay duration")
        .external_details(ExternalPaymentDetails {
            source: "your source".to_owned(),
            source_fee_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            source_id: Some("your source id".to_owned()),
            type_: "your type".to_owned(),
        })
        .location_id("your location id")
        .note("your note")
        .order_id("your order id")
        .reference_id("your reference id")
        .shipping_address(Address {
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
        .statement_description_identifier("your statement description identifier")
        .team_member_id("your team member id")
        .tip_money(Money {
            amount: Some(1),
            currency: Some("your currency".to_owned()),
        })
        .verification_token("your verification token")
        .await
        .unwrap();
    println!("{:#?}", response);
}