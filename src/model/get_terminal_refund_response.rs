
use serde::{Serialize, Deserialize};
use super::{Error, TerminalRefund};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTerminalRefundResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<TerminalRefund>,
}
impl std::fmt::Display for GetTerminalRefundResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}