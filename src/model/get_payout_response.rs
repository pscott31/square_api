
use serde::{Serialize, Deserialize};
use super::{Error, Payout};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetPayoutResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<Payout>,
}
impl std::fmt::Display for GetPayoutResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}