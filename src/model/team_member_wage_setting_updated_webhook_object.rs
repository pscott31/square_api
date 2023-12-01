
use serde::{Serialize, Deserialize};
use super::WageSetting;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TeamMemberWageSettingUpdatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wage_setting: Option<WageSetting>,
}
impl std::fmt::Display for TeamMemberWageSettingUpdatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}