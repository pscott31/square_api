
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenderCashDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_tendered_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_back_money: Option<Money>,
}
impl std::fmt::Display for TenderCashDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}