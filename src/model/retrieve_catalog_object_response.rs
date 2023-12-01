
use serde::{Serialize, Deserialize};
use super::{CatalogObject, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveCatalogObjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<CatalogObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_objects: Option<Vec<CatalogObject>>,
}
impl std::fmt::Display for RetrieveCatalogObjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}