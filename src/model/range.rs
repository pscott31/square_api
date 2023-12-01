
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Range {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
}
impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}