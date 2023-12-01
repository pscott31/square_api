
use serde::{Serialize, Deserialize};
use super::TerminalAction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalActionUpdatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TerminalAction>,
}
impl std::fmt::Display for TerminalActionUpdatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}