
use serde::{Serialize, Deserialize};
use super::{AdditionalRecipient, Address, Order};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Checkout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_recipients: Option<Vec<AdditionalRecipient>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_for_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_page_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_support_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_populate_buyer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_populate_shipping_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}
impl std::fmt::Display for Checkout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}