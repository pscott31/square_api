
use serde::{Serialize, Deserialize};
use super::Shift;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LaborShiftUpdatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<Shift>,
}
impl std::fmt::Display for LaborShiftUpdatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}