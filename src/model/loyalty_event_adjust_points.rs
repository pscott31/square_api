
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyEventAdjustPoints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    pub points: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl std::fmt::Display for LoyaltyEventAdjustPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}