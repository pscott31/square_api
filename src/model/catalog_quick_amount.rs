
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQuickAmount {
    pub amount: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for CatalogQuickAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}