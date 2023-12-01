
use serde::{Serialize, Deserialize};
use super::CatalogQuickAmount;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogQuickAmountsSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<Vec<CatalogQuickAmount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligible_for_auto_amounts: Option<bool>,
    pub option: String,
}
impl std::fmt::Display for CatalogQuickAmountsSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}