
use serde::{Serialize, Deserialize};
use super::{Error, TerminalRefund};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchTerminalRefundsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Vec<TerminalRefund>>,
}
impl std::fmt::Display for SearchTerminalRefundsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}