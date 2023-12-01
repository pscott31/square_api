
use serde::{Serialize, Deserialize};
use super::{Error, Payout};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListPayoutsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<Vec<Payout>>,
}
impl std::fmt::Display for ListPayoutsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}