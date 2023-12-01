
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityImportReversal {
    pub amount_money: Money,
}
impl std::fmt::Display for GiftCardActivityImportReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}