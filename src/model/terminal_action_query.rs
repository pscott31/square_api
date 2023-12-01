
use serde::{Serialize, Deserialize};
use super::{TerminalActionQueryFilter, TerminalActionQuerySort};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalActionQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TerminalActionQueryFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TerminalActionQuerySort>,
}
impl std::fmt::Display for TerminalActionQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}