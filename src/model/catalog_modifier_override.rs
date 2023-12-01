
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogModifierOverride {
    pub modifier_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_by_default: Option<bool>,
}
impl std::fmt::Display for CatalogModifierOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}