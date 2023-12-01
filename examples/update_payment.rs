#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let idempotency_key = "your idempotency key";
    let payment_id = "your payment id";
    let response = client
        .update_payment(idempotency_key, payment_id)
        .payment(Payment {
            amount_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            app_fee_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            application_details: Some(ApplicationDetails {
                application_id: Some("your application id".to_owned()),
                square_product: Some("your square product".to_owned()),
            }),
            approved_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            bank_account_details: Some(BankAccountPaymentDetails {
                account_ownership_type: Some("your account ownership type".to_owned()),
                ach_details: Some(AchDetails {
                    account_number_suffix: Some("your account number suffix".to_owned()),
                    account_type: Some("your account type".to_owned()),
                    routing_number: Some("your routing number".to_owned()),
                }),
                bank_name: Some("your bank name".to_owned()),
                country: Some("your country".to_owned()),
                errors: Some(
                    vec![
                        Error { category : "your category".to_owned(), code : "your code"
                        .to_owned(), detail : Some("your detail".to_owned()), field :
                        Some("your field".to_owned()) }
                    ],
                ),
                fingerprint: Some("your fingerprint".to_owned()),
                statement_description: Some("your statement description".to_owned()),
                transfer_type: Some("your transfer type".to_owned()),
            }),
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
            buy_now_pay_later_details: Some(BuyNowPayLaterDetails {
                afterpay_details: Some(AfterpayDetails {
                    email_address: Some("your email address".to_owned()),
                }),
                brand: Some("your brand".to_owned()),
                clearpay_details: Some(ClearpayDetails {
                    email_address: Some("your email address".to_owned()),
                }),
            }),
            buyer_email_address: Some("your buyer email address".to_owned()),
            capabilities: Some(vec!["your capabilities".to_owned()]),
            card_details: Some(CardPaymentDetails {
                application_cryptogram: Some("your application cryptogram".to_owned()),
                application_identifier: Some("your application identifier".to_owned()),
                application_name: Some("your application name".to_owned()),
                auth_result_code: Some("your auth result code".to_owned()),
                avs_status: Some("your avs status".to_owned()),
                card: Some(Card {
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
                }),
                card_payment_timeline: Some(CardPaymentTimeline {
                    authorized_at: Some("your authorized at".to_owned()),
                    captured_at: Some("your captured at".to_owned()),
                    voided_at: Some("your voided at".to_owned()),
                }),
                cvv_status: Some("your cvv status".to_owned()),
                device_details: Some(DeviceDetails {
                    device_id: Some("your device id".to_owned()),
                    device_installation_id: Some(
                        "your device installation id".to_owned(),
                    ),
                    device_name: Some("your device name".to_owned()),
                }),
                entry_method: Some("your entry method".to_owned()),
                errors: Some(
                    vec![
                        Error { category : "your category".to_owned(), code : "your code"
                        .to_owned(), detail : Some("your detail".to_owned()), field :
                        Some("your field".to_owned()) }
                    ],
                ),
                refund_requires_card_presence: Some(true),
                statement_description: Some("your statement description".to_owned()),
                status: Some("your status".to_owned()),
                verification_method: Some("your verification method".to_owned()),
                verification_results: Some("your verification results".to_owned()),
            }),
            cash_details: Some(CashPaymentDetails {
                buyer_supplied_money: Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                },
                change_back_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
            }),
            created_at: Some("your created at".to_owned()),
            customer_id: Some("your customer id".to_owned()),
            delay_action: Some("your delay action".to_owned()),
            delay_duration: Some("your delay duration".to_owned()),
            delayed_until: Some("your delayed until".to_owned()),
            device_details: Some(DeviceDetails {
                device_id: Some("your device id".to_owned()),
                device_installation_id: Some("your device installation id".to_owned()),
                device_name: Some("your device name".to_owned()),
            }),
            employee_id: Some("your employee id".to_owned()),
            external_details: Some(ExternalPaymentDetails {
                source: "your source".to_owned(),
                source_fee_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
                source_id: Some("your source id".to_owned()),
                type_: "your type".to_owned(),
            }),
            id: Some("your id".to_owned()),
            location_id: Some("your location id".to_owned()),
            note: Some("your note".to_owned()),
            order_id: Some("your order id".to_owned()),
            processing_fee: Some(
                vec![
                    ProcessingFee { amount_money : Some(Money { amount : Some(1),
                    currency : Some("your currency".to_owned()) }), effective_at :
                    Some("your effective at".to_owned()), type_ : Some("your type"
                    .to_owned()) }
                ],
            ),
            receipt_number: Some("your receipt number".to_owned()),
            receipt_url: Some("your receipt url".to_owned()),
            reference_id: Some("your reference id".to_owned()),
            refund_ids: Some(vec!["your refund ids".to_owned()]),
            refunded_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            risk_evaluation: Some(RiskEvaluation {
                created_at: Some("your created at".to_owned()),
                risk_level: Some("your risk level".to_owned()),
            }),
            shipping_address: Some(Address {
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
            source_type: Some("your source type".to_owned()),
            statement_description_identifier: Some(
                "your statement description identifier".to_owned(),
            ),
            status: Some("your status".to_owned()),
            team_member_id: Some("your team member id".to_owned()),
            tip_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            total_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            updated_at: Some("your updated at".to_owned()),
            version_token: Some("your version token".to_owned()),
            wallet_details: Some(DigitalWalletDetails {
                brand: Some("your brand".to_owned()),
                cash_app_details: Some(CashAppDetails {
                    buyer_cashtag: Some("your buyer cashtag".to_owned()),
                    buyer_country_code: Some("your buyer country code".to_owned()),
                    buyer_full_name: Some("your buyer full name".to_owned()),
                }),
                status: Some("your status".to_owned()),
            }),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}