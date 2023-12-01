
use serde::{Serialize, Deserialize};
use super::Card;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenderCardDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl std::fmt::Display for TenderCardDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}