
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingFee {
    pub charge: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for ShippingFee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}