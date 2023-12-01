
use serde::{Serialize, Deserialize};
use super::CatalogObject;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogObjectBatch {
    pub objects: Vec<CatalogObject>,
}
impl std::fmt::Display for CatalogObjectBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}