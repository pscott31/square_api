
use serde::{Serialize, Deserialize};
use super::{Error, Subscription, SubscriptionAction};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SwapPlanResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<SubscriptionAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}
impl std::fmt::Display for SwapPlanResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}