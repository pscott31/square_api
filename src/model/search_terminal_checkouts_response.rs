
use serde::{Serialize, Deserialize};
use super::{Error, TerminalCheckout};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchTerminalCheckoutsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkouts: Option<Vec<TerminalCheckout>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for SearchTerminalCheckoutsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}