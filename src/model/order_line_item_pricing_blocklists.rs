
use serde::{Serialize, Deserialize};
use super::{
    OrderLineItemPricingBlocklistsBlockedDiscount,
    OrderLineItemPricingBlocklistsBlockedTax,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderLineItemPricingBlocklists {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_discounts: Option<Vec<OrderLineItemPricingBlocklistsBlockedDiscount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_taxes: Option<Vec<OrderLineItemPricingBlocklistsBlockedTax>>,
}
impl std::fmt::Display for OrderLineItemPricingBlocklists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}