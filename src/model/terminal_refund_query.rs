
use serde::{Serialize, Deserialize};
use super::{TerminalRefundQueryFilter, TerminalRefundQuerySort};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalRefundQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TerminalRefundQueryFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TerminalRefundQuerySort>,
}
impl std::fmt::Display for TerminalRefundQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}