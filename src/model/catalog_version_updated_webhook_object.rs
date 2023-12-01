
use serde::{Serialize, Deserialize};
use super::CatalogVersionUpdatedWebhookCatalogVersion;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogVersionUpdatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<CatalogVersionUpdatedWebhookCatalogVersion>,
}
impl std::fmt::Display for CatalogVersionUpdatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}