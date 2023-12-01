
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct V1PaymentItemDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_variation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
}
impl std::fmt::Display for V1PaymentItemDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}