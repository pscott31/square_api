
use serde::{Serialize, Deserialize};
use super::LoyaltyEventFilter;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyEventQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<LoyaltyEventFilter>,
}
impl std::fmt::Display for LoyaltyEventQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}