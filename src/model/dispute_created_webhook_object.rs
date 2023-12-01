
use serde::{Serialize, Deserialize};
use super::Dispute;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputeCreatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Dispute>,
}
impl std::fmt::Display for DisputeCreatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}