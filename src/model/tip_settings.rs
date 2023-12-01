
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TipSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tipping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_tip_field: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate_tip_screen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tipping: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_percentages: Option<Vec<i64>>,
}
impl std::fmt::Display for TipSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}