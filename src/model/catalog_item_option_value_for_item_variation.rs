
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogItemOptionValueForItemVariation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_option_value_id: Option<String>,
}
impl std::fmt::Display for CatalogItemOptionValueForItemVariation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}