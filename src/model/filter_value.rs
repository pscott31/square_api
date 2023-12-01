
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilterValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none: Option<Vec<String>>,
}
impl std::fmt::Display for FilterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}