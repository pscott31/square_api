
use serde::{Serialize, Deserialize};
use super::Card;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardAutomaticallyUpdatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
}
impl std::fmt::Display for CardAutomaticallyUpdatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}