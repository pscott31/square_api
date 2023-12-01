
use serde::{Serialize, Deserialize};
use super::Customer;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerUpdatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
}
impl std::fmt::Display for CustomerUpdatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}