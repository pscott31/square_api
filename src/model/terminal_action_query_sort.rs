
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalActionQuerySort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}
impl std::fmt::Display for TerminalActionQuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}