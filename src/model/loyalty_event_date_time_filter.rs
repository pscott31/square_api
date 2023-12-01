
use serde::{Serialize, Deserialize};
use super::TimeRange;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventDateTimeFilter {
    pub created_at: TimeRange,
}
impl std::fmt::Display for LoyaltyEventDateTimeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}