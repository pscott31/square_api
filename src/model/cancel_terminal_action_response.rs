
use serde::{Serialize, Deserialize};
use super::{Error, TerminalAction};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CancelTerminalActionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TerminalAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for CancelTerminalActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}