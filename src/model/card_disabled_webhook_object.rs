
use serde::{Serialize, Deserialize};
use super::Card;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardDisabledWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
}
impl std::fmt::Display for CardDisabledWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}