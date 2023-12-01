
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyAccountExpiringPointDeadline {
    pub expires_at: String,
    pub points: i64,
}
impl std::fmt::Display for LoyaltyAccountExpiringPointDeadline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}