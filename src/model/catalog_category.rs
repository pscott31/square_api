
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for CatalogCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}