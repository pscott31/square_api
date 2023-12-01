
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogQueryItemsForItemOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_option_ids: Option<Vec<String>>,
}
impl std::fmt::Display for CatalogQueryItemsForItemOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}