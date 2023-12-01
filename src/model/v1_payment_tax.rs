
use serde::{Serialize, Deserialize};
use super::{Error, V1Money};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct V1PaymentTax {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_money: Option<V1Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
}
impl std::fmt::Display for V1PaymentTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}