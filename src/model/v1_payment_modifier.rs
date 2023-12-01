
use serde::{Serialize, Deserialize};
use super::V1Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct V1PaymentModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for V1PaymentModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}