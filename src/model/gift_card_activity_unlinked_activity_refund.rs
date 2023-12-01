
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityUnlinkedActivityRefund {
    pub amount_money: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
}
impl std::fmt::Display for GiftCardActivityUnlinkedActivityRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}