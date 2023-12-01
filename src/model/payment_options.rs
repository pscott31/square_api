
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_partial_authorization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_duration: Option<String>,
}
impl std::fmt::Display for PaymentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}