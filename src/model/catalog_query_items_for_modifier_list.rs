
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogQueryItemsForModifierList {
    pub modifier_list_ids: Vec<String>,
}
impl std::fmt::Display for CatalogQueryItemsForModifierList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}