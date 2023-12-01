
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderLineItemPricingBlocklistsBlockedTax {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_catalog_object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl std::fmt::Display for OrderLineItemPricingBlocklistsBlockedTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}