
use serde::{Serialize, Deserialize};
use super::{Error, PayoutEntry};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListPayoutEntriesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_entries: Option<Vec<PayoutEntry>>,
}
impl std::fmt::Display for ListPayoutEntriesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}