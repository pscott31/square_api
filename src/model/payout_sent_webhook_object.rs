
use serde::{Serialize, Deserialize};
use super::Payout;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayoutSentWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<Payout>,
}
impl std::fmt::Display for PayoutSentWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}