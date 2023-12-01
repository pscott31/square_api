
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerTextFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<String>,
}
impl std::fmt::Display for CustomerTextFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}