
use serde::{Serialize, Deserialize};
use super::CardForgottenWebhookCard;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardForgottenWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardForgottenWebhookCard>,
}
impl std::fmt::Display for CardForgottenWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}