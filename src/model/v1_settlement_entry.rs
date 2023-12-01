
use serde::{Serialize, Deserialize};
use super::V1Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct V1SettlementEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for V1SettlementEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}