
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderPricingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_apply_discounts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_apply_taxes: Option<bool>,
}
impl std::fmt::Display for OrderPricingOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}