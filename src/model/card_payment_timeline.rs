
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardPaymentTimeline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captured_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voided_at: Option<String>,
}
impl std::fmt::Display for CardPaymentTimeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}