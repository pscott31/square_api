
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_list_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_money: Option<Money>,
}
impl std::fmt::Display for CatalogModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}