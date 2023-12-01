
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogQueryRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_max_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_min_value: Option<i64>,
    pub attribute_name: String,
}
impl std::fmt::Display for CatalogQueryRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}