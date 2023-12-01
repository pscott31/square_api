#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let invoice = Invoice {
        accepted_payment_methods: Some(InvoiceAcceptedPaymentMethods {
            bank_account: Some(true),
            card: Some(true),
            square_gift_card: Some(true),
        }),
        created_at: Some("your created at".to_owned()),
        custom_fields: Some(
            vec![
                InvoiceCustomField { label : Some("your label".to_owned()), placement :
                Some("your placement".to_owned()), value : Some("your value".to_owned())
                }
            ],
        ),
        delivery_method: Some("your delivery method".to_owned()),
        description: Some("your description".to_owned()),
        id: Some("your id".to_owned()),
        invoice_number: Some("your invoice number".to_owned()),
        location_id: Some("your location id".to_owned()),
        next_payment_amount_money: Some(Money {
            amount: Some(1),
            currency: Some("your currency".to_owned()),
        }),
        order_id: Some("your order id".to_owned()),
        payment_conditions: Some("your payment conditions".to_owned()),
        payment_requests: Some(
            vec![
                InvoicePaymentRequest { automatic_payment_source :
                Some("your automatic payment source".to_owned()), card_id :
                Some("your card id".to_owned()), computed_amount_money : Some(Money {
                amount : Some(1), currency : Some("your currency".to_owned()) }),
                due_date : Some("your due date".to_owned()), fixed_amount_requested_money
                : Some(Money { amount : Some(1), currency : Some("your currency"
                .to_owned()) }), percentage_requested : Some("your percentage requested"
                .to_owned()), reminders : Some(vec![InvoicePaymentReminder { message :
                Some("your message".to_owned()), relative_scheduled_days : Some(1),
                sent_at : Some("your sent at".to_owned()), status : Some("your status"
                .to_owned()), uid : Some("your uid".to_owned()) }]), request_method :
                Some("your request method".to_owned()), request_type :
                Some("your request type".to_owned()), rounding_adjustment_included_money
                : Some(Money { amount : Some(1), currency : Some("your currency"
                .to_owned()) }), tipping_enabled : Some(true),
                total_completed_amount_money : Some(Money { amount : Some(1), currency :
                Some("your currency".to_owned()) }), uid : Some("your uid".to_owned()) }
            ],
        ),
        primary_recipient: Some(InvoiceRecipient {
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
            company_name: Some("your company name".to_owned()),
            customer_id: Some("your customer id".to_owned()),
            email_address: Some("your email address".to_owned()),
            family_name: Some("your family name".to_owned()),
            given_name: Some("your given name".to_owned()),
            phone_number: Some("your phone number".to_owned()),
            tax_ids: Some(InvoiceRecipientTaxIds {
                eu_vat: Some("your eu vat".to_owned()),
            }),
        }),
        public_url: Some("your public url".to_owned()),
        sale_or_service_date: Some("your sale or service date".to_owned()),
        scheduled_at: Some("your scheduled at".to_owned()),
        status: Some("your status".to_owned()),
        subscription_id: Some("your subscription id".to_owned()),
        timezone: Some("your timezone".to_owned()),
        title: Some("your title".to_owned()),
        updated_at: Some("your updated at".to_owned()),
        version: Some(1),
    };
    let invoice_id = "your invoice id";
    let response = client
        .update_invoice(invoice, invoice_id)
        .fields_to_clear(&["your fields to clear"])
        .idempotency_key("your idempotency key")
        .await
        .unwrap();
    println!("{:#?}", response);
}