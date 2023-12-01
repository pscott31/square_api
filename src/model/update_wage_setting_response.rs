
use serde::{Serialize, Deserialize};
use super::{Error, WageSetting};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateWageSettingResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wage_setting: Option<WageSetting>,
}
impl std::fmt::Display for UpdateWageSettingResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}