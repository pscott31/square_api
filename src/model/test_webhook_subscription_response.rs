
use serde::{Serialize, Deserialize};
use super::{Error, SubscriptionTestResult};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestWebhookSubscriptionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_test_result: Option<SubscriptionTestResult>,
}
impl std::fmt::Display for TestWebhookSubscriptionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}