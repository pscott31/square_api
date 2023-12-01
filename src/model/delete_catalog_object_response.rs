
use serde::{Serialize, Deserialize};
use super::Error;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteCatalogObjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_object_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for DeleteCatalogObjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}