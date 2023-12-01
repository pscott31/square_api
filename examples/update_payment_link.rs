#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let id = "your id";
    let payment_link = PaymentLink {
        checkout_options: Some(CheckoutOptions {
            accepted_payment_methods: Some(AcceptedPaymentMethods {
                afterpay_clearpay: Some(true),
                apple_pay: Some(true),
                cash_app_pay: Some(true),
                google_pay: Some(true),
            }),
            allow_tipping: Some(true),
            app_fee_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            ask_for_shipping_address: Some(true),
            custom_fields: Some(vec![CustomField { title : "your title".to_owned() }]),
            merchant_support_email: Some("your merchant support email".to_owned()),
            redirect_url: Some("your redirect url".to_owned()),
            shipping_fee: Some(ShippingFee {
                charge: Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                },
                name: Some("your name".to_owned()),
            }),
            subscription_plan_id: Some("your subscription plan id".to_owned()),
        }),
        created_at: Some("your created at".to_owned()),
        description: Some("your description".to_owned()),
        id: Some("your id".to_owned()),
        order_id: Some("your order id".to_owned()),
        payment_note: Some("your payment note".to_owned()),
        pre_populated_data: Some(PrePopulatedData {
            buyer_address: Some(Address {
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
            buyer_email: Some("your buyer email".to_owned()),
            buyer_phone_number: Some("your buyer phone number".to_owned()),
        }),
        updated_at: Some("your updated at".to_owned()),
        url: Some("your url".to_owned()),
        version: 1,
    };
    let response = client.update_payment_link(id, payment_link).await.unwrap();
    println!("{:#?}", response);
}