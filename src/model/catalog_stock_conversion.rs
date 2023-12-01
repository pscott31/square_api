
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogStockConversion {
    pub nonstockable_quantity: String,
    pub stockable_item_variation_id: String,
    pub stockable_quantity: String,
}
impl std::fmt::Display for CatalogStockConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}