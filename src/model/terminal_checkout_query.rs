
use serde::{Serialize, Deserialize};
use super::{TerminalCheckoutQueryFilter, TerminalCheckoutQuerySort};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalCheckoutQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TerminalCheckoutQueryFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TerminalCheckoutQuerySort>,
}
impl std::fmt::Display for TerminalCheckoutQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}