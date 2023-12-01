
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AchDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
impl std::fmt::Display for AchDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}