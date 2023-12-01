
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityAdjustDecrement {
    pub amount_money: Money,
    pub reason: String,
}
impl std::fmt::Display for GiftCardActivityAdjustDecrement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}