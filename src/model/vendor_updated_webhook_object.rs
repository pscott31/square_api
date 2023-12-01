
use serde::{Serialize, Deserialize};
use super::Vendor;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VendorUpdatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<Vendor>,
}
impl std::fmt::Display for VendorUpdatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}