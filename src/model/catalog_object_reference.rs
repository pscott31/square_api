
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogObjectReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
impl std::fmt::Display for CatalogObjectReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}