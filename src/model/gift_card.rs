
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GiftCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gan_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for GiftCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}