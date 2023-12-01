
use serde::{Serialize, Deserialize};
use super::{CatalogObject, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCatalogImageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CatalogObject>,
}
impl std::fmt::Display for CreateCatalogImageResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}