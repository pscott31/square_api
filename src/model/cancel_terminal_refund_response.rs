
use serde::{Serialize, Deserialize};
use super::{Error, TerminalRefund};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CancelTerminalRefundResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<TerminalRefund>,
}
impl std::fmt::Display for CancelTerminalRefundResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}