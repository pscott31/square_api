
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogProductSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_products: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids_all: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids_any: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_exact: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_min: Option<i64>,
}
impl std::fmt::Display for CatalogProductSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}