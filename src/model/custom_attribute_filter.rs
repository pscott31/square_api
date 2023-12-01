
use serde::{Serialize, Deserialize};
use super::Range;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomAttributeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_filter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attribute_definition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_filter: Option<Range>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_uids_filter: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_filter: Option<String>,
}
impl std::fmt::Display for CustomAttributeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}