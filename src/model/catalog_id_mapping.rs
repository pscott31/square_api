
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogIdMapping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
impl std::fmt::Display for CatalogIdMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}