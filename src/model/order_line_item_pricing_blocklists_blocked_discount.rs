
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderLineItemPricingBlocklistsBlockedDiscount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_catalog_object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl std::fmt::Display for OrderLineItemPricingBlocklistsBlockedDiscount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}