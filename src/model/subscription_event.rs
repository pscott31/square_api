
use serde::{Serialize, Deserialize};
use super::SubscriptionEventInfo;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionEvent {
    pub effective_date: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<SubscriptionEventInfo>,
    pub plan_id: String,
    pub subscription_event_type: String,
}
impl std::fmt::Display for SubscriptionEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}