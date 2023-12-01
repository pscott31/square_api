#![allow(unused_imports)]
use square_api::SquareApiClient;
use square_api::model::*;
#[tokio::main]
async fn main() {
    let client = SquareApiClient::from_env();
    let response = client
        .create_payment_link()
        .checkout_options(CheckoutOptions {
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
        })
        .description("your description")
        .idempotency_key("your idempotency key")
        .order(Order {
            closed_at: Some("your closed at".to_owned()),
            created_at: Some("your created at".to_owned()),
            customer_id: Some("your customer id".to_owned()),
            discounts: Some(
                vec![
                    OrderLineItemDiscount { amount_money : Some(Money { amount : Some(1),
                    currency : Some("your currency".to_owned()) }), applied_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), catalog_object_id : Some("your catalog object id"
                    .to_owned()), catalog_version : Some(1), metadata :
                    Some(serde_json::json!({})), name : Some("your name".to_owned()),
                    percentage : Some("your percentage".to_owned()), pricing_rule_id :
                    Some("your pricing rule id".to_owned()), reward_ids :
                    Some(vec!["your reward ids".to_owned()]), scope : Some("your scope"
                    .to_owned()), type_ : Some("your type".to_owned()), uid :
                    Some("your uid".to_owned()) }
                ],
            ),
            fulfillments: Some(
                vec![
                    OrderFulfillment { entries :
                    Some(vec![OrderFulfillmentFulfillmentEntry { line_item_uid :
                    "your line item uid".to_owned(), metadata :
                    Some(serde_json::json!({})), quantity : "your quantity".to_owned(),
                    uid : Some("your uid".to_owned()) }]), line_item_application :
                    Some("your line item application".to_owned()), metadata :
                    Some(serde_json::json!({})), pickup_details :
                    Some(OrderFulfillmentPickupDetails { accepted_at :
                    Some("your accepted at".to_owned()), auto_complete_duration :
                    Some("your auto complete duration".to_owned()), cancel_reason :
                    Some("your cancel reason".to_owned()), canceled_at :
                    Some("your canceled at".to_owned()), curbside_pickup_details :
                    Some(OrderFulfillmentPickupDetailsCurbsidePickupDetails {
                    buyer_arrived_at : Some("your buyer arrived at".to_owned()),
                    curbside_details : Some("your curbside details".to_owned()) }),
                    expired_at : Some("your expired at".to_owned()), expires_at :
                    Some("your expires at".to_owned()), is_curbside_pickup : Some(true),
                    note : Some("your note".to_owned()), picked_up_at :
                    Some("your picked up at".to_owned()), pickup_at :
                    Some("your pickup at".to_owned()), pickup_window_duration :
                    Some("your pickup window duration".to_owned()), placed_at :
                    Some("your placed at".to_owned()), prep_time_duration :
                    Some("your prep time duration".to_owned()), ready_at :
                    Some("your ready at".to_owned()), recipient :
                    Some(OrderFulfillmentRecipient { address : Some(Address {
                    address_line1 : Some("your address line 1".to_owned()), address_line2
                    : Some("your address line 2".to_owned()), address_line3 :
                    Some("your address line 3".to_owned()),
                    administrative_district_level1 :
                    Some("your administrative district level 1".to_owned()),
                    administrative_district_level2 :
                    Some("your administrative district level 2".to_owned()),
                    administrative_district_level3 :
                    Some("your administrative district level 3".to_owned()), country :
                    Some("your country".to_owned()), first_name : Some("your first name"
                    .to_owned()), last_name : Some("your last name".to_owned()), locality
                    : Some("your locality".to_owned()), postal_code :
                    Some("your postal code".to_owned()), sublocality :
                    Some("your sublocality".to_owned()), sublocality2 :
                    Some("your sublocality 2".to_owned()), sublocality3 :
                    Some("your sublocality 3".to_owned()) }), customer_id :
                    Some("your customer id".to_owned()), display_name :
                    Some("your display name".to_owned()), email_address :
                    Some("your email address".to_owned()), phone_number :
                    Some("your phone number".to_owned()) }), rejected_at :
                    Some("your rejected at".to_owned()), schedule_type :
                    Some("your schedule type".to_owned()) }), shipment_details :
                    Some(OrderFulfillmentShipmentDetails { cancel_reason :
                    Some("your cancel reason".to_owned()), canceled_at :
                    Some("your canceled at".to_owned()), carrier : Some("your carrier"
                    .to_owned()), expected_shipped_at : Some("your expected shipped at"
                    .to_owned()), failed_at : Some("your failed at".to_owned()),
                    failure_reason : Some("your failure reason".to_owned()),
                    in_progress_at : Some("your in progress at".to_owned()), packaged_at
                    : Some("your packaged at".to_owned()), placed_at :
                    Some("your placed at".to_owned()), recipient :
                    Some(OrderFulfillmentRecipient { address : Some(Address {
                    address_line1 : Some("your address line 1".to_owned()), address_line2
                    : Some("your address line 2".to_owned()), address_line3 :
                    Some("your address line 3".to_owned()),
                    administrative_district_level1 :
                    Some("your administrative district level 1".to_owned()),
                    administrative_district_level2 :
                    Some("your administrative district level 2".to_owned()),
                    administrative_district_level3 :
                    Some("your administrative district level 3".to_owned()), country :
                    Some("your country".to_owned()), first_name : Some("your first name"
                    .to_owned()), last_name : Some("your last name".to_owned()), locality
                    : Some("your locality".to_owned()), postal_code :
                    Some("your postal code".to_owned()), sublocality :
                    Some("your sublocality".to_owned()), sublocality2 :
                    Some("your sublocality 2".to_owned()), sublocality3 :
                    Some("your sublocality 3".to_owned()) }), customer_id :
                    Some("your customer id".to_owned()), display_name :
                    Some("your display name".to_owned()), email_address :
                    Some("your email address".to_owned()), phone_number :
                    Some("your phone number".to_owned()) }), shipped_at :
                    Some("your shipped at".to_owned()), shipping_note :
                    Some("your shipping note".to_owned()), shipping_type :
                    Some("your shipping type".to_owned()), tracking_number :
                    Some("your tracking number".to_owned()), tracking_url :
                    Some("your tracking url".to_owned()) }), state : Some("your state"
                    .to_owned()), type_ : Some("your type".to_owned()), uid :
                    Some("your uid".to_owned()) }
                ],
            ),
            id: Some("your id".to_owned()),
            line_items: Some(
                vec![
                    OrderLineItem { applied_discounts :
                    Some(vec![OrderLineItemAppliedDiscount { applied_money : Some(Money {
                    amount : Some(1), currency : Some("your currency".to_owned()) }),
                    discount_uid : "your discount uid".to_owned(), uid : Some("your uid"
                    .to_owned()) }]), applied_taxes : Some(vec![OrderLineItemAppliedTax {
                    applied_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }), tax_uid : "your tax uid"
                    .to_owned(), uid : Some("your uid".to_owned()) }]), base_price_money
                    : Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), catalog_object_id : Some("your catalog object id"
                    .to_owned()), catalog_version : Some(1), gross_sales_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), item_type : Some("your item type".to_owned()),
                    metadata : Some(serde_json::json!({})), modifiers :
                    Some(vec![OrderLineItemModifier { base_price_money : Some(Money {
                    amount : Some(1), currency : Some("your currency".to_owned()) }),
                    catalog_object_id : Some("your catalog object id".to_owned()),
                    catalog_version : Some(1), metadata : Some(serde_json::json!({})),
                    name : Some("your name".to_owned()), quantity : Some("your quantity"
                    .to_owned()), total_price_money : Some(Money { amount : Some(1),
                    currency : Some("your currency".to_owned()) }), uid : Some("your uid"
                    .to_owned()) }]), name : Some("your name".to_owned()), note :
                    Some("your note".to_owned()), pricing_blocklists :
                    Some(OrderLineItemPricingBlocklists { blocked_discounts :
                    Some(vec![OrderLineItemPricingBlocklistsBlockedDiscount {
                    discount_catalog_object_id : Some("your discount catalog object id"
                    .to_owned()), discount_uid : Some("your discount uid".to_owned()),
                    uid : Some("your uid".to_owned()) }]), blocked_taxes :
                    Some(vec![OrderLineItemPricingBlocklistsBlockedTax {
                    tax_catalog_object_id : Some("your tax catalog object id"
                    .to_owned()), tax_uid : Some("your tax uid".to_owned()), uid :
                    Some("your uid".to_owned()) }]) }), quantity : "your quantity"
                    .to_owned(), quantity_unit : Some(OrderQuantityUnit {
                    catalog_object_id : Some("your catalog object id".to_owned()),
                    catalog_version : Some(1), measurement_unit : Some(MeasurementUnit {
                    area_unit : Some("your area unit".to_owned()), custom_unit :
                    Some(MeasurementUnitCustom { abbreviation : "your abbreviation"
                    .to_owned(), name : "your name".to_owned() }), generic_unit :
                    Some("your generic unit".to_owned()), length_unit :
                    Some("your length unit".to_owned()), time_unit :
                    Some("your time unit".to_owned()), type_ : Some("your type"
                    .to_owned()), volume_unit : Some("your volume unit".to_owned()),
                    weight_unit : Some("your weight unit".to_owned()) }), precision :
                    Some(1) }), total_discount_money : Some(Money { amount : Some(1),
                    currency : Some("your currency".to_owned()) }), total_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), total_tax_money : Some(Money { amount : Some(1),
                    currency : Some("your currency".to_owned()) }), uid : Some("your uid"
                    .to_owned()), variation_name : Some("your variation name"
                    .to_owned()), variation_total_price_money : Some(Money { amount :
                    Some(1), currency : Some("your currency".to_owned()) }) }
                ],
            ),
            location_id: "your location id".to_owned(),
            metadata: Some(serde_json::json!({})),
            net_amount_due_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            net_amounts: Some(OrderMoneyAmounts {
                discount_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
                service_charge_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
                tax_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
                tip_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
                total_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
            }),
            pricing_options: Some(OrderPricingOptions {
                auto_apply_discounts: Some(true),
                auto_apply_taxes: Some(true),
            }),
            reference_id: Some("your reference id".to_owned()),
            refunds: Some(
                vec![
                    Refund { additional_recipients : Some(vec![AdditionalRecipient {
                    amount_money : Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }, description :
                    Some("your description".to_owned()), location_id : "your location id"
                    .to_owned(), receivable_id : Some("your receivable id".to_owned())
                    }]), amount_money : Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }, created_at :
                    Some("your created at".to_owned()), id : "your id".to_owned(),
                    location_id : "your location id".to_owned(), processing_fee_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), reason : "your reason".to_owned(), status :
                    "your status".to_owned(), tender_id : "your tender id".to_owned(),
                    transaction_id : Some("your transaction id".to_owned()) }
                ],
            ),
            return_amounts: Some(OrderMoneyAmounts {
                discount_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
                service_charge_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
                tax_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
                tip_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
                total_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
            }),
            returns: Some(
                vec![
                    OrderReturn { return_amounts : Some(OrderMoneyAmounts {
                    discount_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }), service_charge_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), tax_money : Some(Money { amount : Some(1), currency
                    : Some("your currency".to_owned()) }), tip_money : Some(Money {
                    amount : Some(1), currency : Some("your currency".to_owned()) }),
                    total_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }) }), return_discounts :
                    Some(vec![OrderReturnDiscount { amount_money : Some(Money { amount :
                    Some(1), currency : Some("your currency".to_owned()) }),
                    applied_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }), catalog_object_id :
                    Some("your catalog object id".to_owned()), catalog_version : Some(1),
                    name : Some("your name".to_owned()), percentage :
                    Some("your percentage".to_owned()), scope : Some("your scope"
                    .to_owned()), source_discount_uid : Some("your source discount uid"
                    .to_owned()), type_ : Some("your type".to_owned()), uid :
                    Some("your uid".to_owned()) }]), return_line_items :
                    Some(vec![OrderReturnLineItem { applied_discounts :
                    Some(vec![OrderLineItemAppliedDiscount { applied_money : Some(Money {
                    amount : Some(1), currency : Some("your currency".to_owned()) }),
                    discount_uid : "your discount uid".to_owned(), uid : Some("your uid"
                    .to_owned()) }]), applied_taxes : Some(vec![OrderLineItemAppliedTax {
                    applied_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }), tax_uid : "your tax uid"
                    .to_owned(), uid : Some("your uid".to_owned()) }]), base_price_money
                    : Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), catalog_object_id : Some("your catalog object id"
                    .to_owned()), catalog_version : Some(1), gross_return_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), item_type : Some("your item type".to_owned()), name
                    : Some("your name".to_owned()), note : Some("your note".to_owned()),
                    quantity : "your quantity".to_owned(), quantity_unit :
                    Some(OrderQuantityUnit { catalog_object_id :
                    Some("your catalog object id".to_owned()), catalog_version : Some(1),
                    measurement_unit : Some(MeasurementUnit { area_unit :
                    Some("your area unit".to_owned()), custom_unit :
                    Some(MeasurementUnitCustom { abbreviation : "your abbreviation"
                    .to_owned(), name : "your name".to_owned() }), generic_unit :
                    Some("your generic unit".to_owned()), length_unit :
                    Some("your length unit".to_owned()), time_unit :
                    Some("your time unit".to_owned()), type_ : Some("your type"
                    .to_owned()), volume_unit : Some("your volume unit".to_owned()),
                    weight_unit : Some("your weight unit".to_owned()) }), precision :
                    Some(1) }), return_modifiers : Some(vec![OrderReturnLineItemModifier
                    { base_price_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }), catalog_object_id :
                    Some("your catalog object id".to_owned()), catalog_version : Some(1),
                    name : Some("your name".to_owned()), source_modifier_uid :
                    Some("your source modifier uid".to_owned()), total_price_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), uid : Some("your uid".to_owned()) }]),
                    source_line_item_uid : Some("your source line item uid".to_owned()),
                    total_discount_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }), total_money : Some(Money {
                    amount : Some(1), currency : Some("your currency".to_owned()) }),
                    total_tax_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }), uid : Some("your uid"
                    .to_owned()), variation_name : Some("your variation name"
                    .to_owned()), variation_total_price_money : Some(Money { amount :
                    Some(1), currency : Some("your currency".to_owned()) }) }]),
                    return_service_charges : Some(vec![OrderReturnServiceCharge {
                    amount_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }), applied_money : Some(Money {
                    amount : Some(1), currency : Some("your currency".to_owned()) }),
                    applied_taxes : Some(vec![OrderLineItemAppliedTax { applied_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), tax_uid : "your tax uid".to_owned(), uid :
                    Some("your uid".to_owned()) }]), calculation_phase :
                    Some("your calculation phase".to_owned()), catalog_object_id :
                    Some("your catalog object id".to_owned()), catalog_version : Some(1),
                    name : Some("your name".to_owned()), percentage :
                    Some("your percentage".to_owned()), source_service_charge_uid :
                    Some("your source service charge uid".to_owned()), taxable :
                    Some(true), total_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }), total_tax_money : Some(Money {
                    amount : Some(1), currency : Some("your currency".to_owned()) }), uid
                    : Some("your uid".to_owned()) }]), return_taxes :
                    Some(vec![OrderReturnTax { applied_money : Some(Money { amount :
                    Some(1), currency : Some("your currency".to_owned()) }),
                    catalog_object_id : Some("your catalog object id".to_owned()),
                    catalog_version : Some(1), name : Some("your name".to_owned()),
                    percentage : Some("your percentage".to_owned()), scope :
                    Some("your scope".to_owned()), source_tax_uid :
                    Some("your source tax uid".to_owned()), type_ : Some("your type"
                    .to_owned()), uid : Some("your uid".to_owned()) }]),
                    rounding_adjustment : Some(OrderRoundingAdjustment { amount_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), name : Some("your name".to_owned()), uid :
                    Some("your uid".to_owned()) }), source_order_id :
                    Some("your source order id".to_owned()), uid : Some("your uid"
                    .to_owned()) }
                ],
            ),
            rewards: Some(
                vec![
                    OrderReward { id : "your id".to_owned(), reward_tier_id :
                    "your reward tier id".to_owned() }
                ],
            ),
            rounding_adjustment: Some(OrderRoundingAdjustment {
                amount_money: Some(Money {
                    amount: Some(1),
                    currency: Some("your currency".to_owned()),
                }),
                name: Some("your name".to_owned()),
                uid: Some("your uid".to_owned()),
            }),
            service_charges: Some(
                vec![
                    OrderServiceCharge { amount_money : Some(Money { amount : Some(1),
                    currency : Some("your currency".to_owned()) }), applied_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), applied_taxes : Some(vec![OrderLineItemAppliedTax {
                    applied_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }), tax_uid : "your tax uid"
                    .to_owned(), uid : Some("your uid".to_owned()) }]), calculation_phase
                    : Some("your calculation phase".to_owned()), catalog_object_id :
                    Some("your catalog object id".to_owned()), catalog_version : Some(1),
                    metadata : Some(serde_json::json!({})), name : Some("your name"
                    .to_owned()), percentage : Some("your percentage".to_owned()),
                    taxable : Some(true), total_money : Some(Money { amount : Some(1),
                    currency : Some("your currency".to_owned()) }), total_tax_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), type_ : Some("your type".to_owned()), uid :
                    Some("your uid".to_owned()) }
                ],
            ),
            source: Some(OrderSource {
                name: Some("your name".to_owned()),
            }),
            state: Some("your state".to_owned()),
            taxes: Some(
                vec![
                    OrderLineItemTax { applied_money : Some(Money { amount : Some(1),
                    currency : Some("your currency".to_owned()) }), auto_applied :
                    Some(true), catalog_object_id : Some("your catalog object id"
                    .to_owned()), catalog_version : Some(1), metadata :
                    Some(serde_json::json!({})), name : Some("your name".to_owned()),
                    percentage : Some("your percentage".to_owned()), scope :
                    Some("your scope".to_owned()), type_ : Some("your type".to_owned()),
                    uid : Some("your uid".to_owned()) }
                ],
            ),
            tenders: Some(
                vec![
                    Tender { additional_recipients : Some(vec![AdditionalRecipient {
                    amount_money : Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }, description :
                    Some("your description".to_owned()), location_id : "your location id"
                    .to_owned(), receivable_id : Some("your receivable id".to_owned())
                    }]), amount_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }), card_details :
                    Some(TenderCardDetails { card : Some(Card { billing_address :
                    Some(Address { address_line1 : Some("your address line 1"
                    .to_owned()), address_line2 : Some("your address line 2".to_owned()),
                    address_line3 : Some("your address line 3".to_owned()),
                    administrative_district_level1 :
                    Some("your administrative district level 1".to_owned()),
                    administrative_district_level2 :
                    Some("your administrative district level 2".to_owned()),
                    administrative_district_level3 :
                    Some("your administrative district level 3".to_owned()), country :
                    Some("your country".to_owned()), first_name : Some("your first name"
                    .to_owned()), last_name : Some("your last name".to_owned()), locality
                    : Some("your locality".to_owned()), postal_code :
                    Some("your postal code".to_owned()), sublocality :
                    Some("your sublocality".to_owned()), sublocality2 :
                    Some("your sublocality 2".to_owned()), sublocality3 :
                    Some("your sublocality 3".to_owned()) }), bin : Some("your bin"
                    .to_owned()), card_brand : Some("your card brand".to_owned()),
                    card_co_brand : Some("your card co brand".to_owned()), card_type :
                    Some("your card type".to_owned()), cardholder_name :
                    Some("your cardholder name".to_owned()), customer_id :
                    Some("your customer id".to_owned()), enabled : Some(true), exp_month
                    : Some(1), exp_year : Some(1), fingerprint : Some("your fingerprint"
                    .to_owned()), id : Some("your id".to_owned()), last4 :
                    Some("your last 4".to_owned()), merchant_id : Some("your merchant id"
                    .to_owned()), prepaid_type : Some("your prepaid type".to_owned()),
                    reference_id : Some("your reference id".to_owned()), version :
                    Some(1) }), entry_method : Some("your entry method".to_owned()),
                    status : Some("your status".to_owned()) }), cash_details :
                    Some(TenderCashDetails { buyer_tendered_money : Some(Money { amount :
                    Some(1), currency : Some("your currency".to_owned()) }),
                    change_back_money : Some(Money { amount : Some(1), currency :
                    Some("your currency".to_owned()) }) }), created_at :
                    Some("your created at".to_owned()), customer_id :
                    Some("your customer id".to_owned()), id : Some("your id".to_owned()),
                    location_id : Some("your location id".to_owned()), note :
                    Some("your note".to_owned()), payment_id : Some("your payment id"
                    .to_owned()), processing_fee_money : Some(Money { amount : Some(1),
                    currency : Some("your currency".to_owned()) }), tip_money :
                    Some(Money { amount : Some(1), currency : Some("your currency"
                    .to_owned()) }), transaction_id : Some("your transaction id"
                    .to_owned()), type_ : "your type".to_owned() }
                ],
            ),
            ticket_name: Some("your ticket name".to_owned()),
            total_discount_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            total_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            total_service_charge_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            total_tax_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            total_tip_money: Some(Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            }),
            updated_at: Some("your updated at".to_owned()),
            version: Some(1),
        })
        .payment_note("your payment note")
        .pre_populated_data(PrePopulatedData {
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
        })
        .quick_pay(QuickPay {
            location_id: "your location id".to_owned(),
            name: "your name".to_owned(),
            price_money: Money {
                amount: Some(1),
                currency: Some("your currency".to_owned()),
            },
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}