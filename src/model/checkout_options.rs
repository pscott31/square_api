
use serde::{Serialize, Deserialize};
use super::{AcceptedPaymentMethods, CustomField, Money, ShippingFee};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckoutOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_payment_methods: Option<AcceptedPaymentMethods>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tipping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_fee_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_for_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_support_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_fee: Option<ShippingFee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_plan_id: Option<String>,
}
impl std::fmt::Display for CheckoutOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}