
use serde::{Serialize, Deserialize};
use super::{CatalogIdMapping, CatalogObject, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchUpsertCatalogObjectsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_mappings: Option<Vec<CatalogIdMapping>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<CatalogObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl std::fmt::Display for BatchUpsertCatalogObjectsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}