
use serde::{Serialize, Deserialize};
use super::LoyaltyProgram;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyProgramCreatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program: Option<LoyaltyProgram>,
}
impl std::fmt::Display for LoyaltyProgramCreatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}