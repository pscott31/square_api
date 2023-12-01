
use serde::{Serialize, Deserialize};
use super::{CatalogObject, Order};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinkRelatedResources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<Order>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_plans: Option<Vec<CatalogObject>>,
}
impl std::fmt::Display for PaymentLinkRelatedResources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}