
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyEventExpirePoints {
    pub loyalty_program_id: String,
    pub points: i64,
}
impl std::fmt::Display for LoyaltyEventExpirePoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}