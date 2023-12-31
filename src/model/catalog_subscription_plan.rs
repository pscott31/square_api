
use serde::{Serialize, Deserialize};
use super::SubscriptionPhase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogSubscriptionPlan {
    pub name: String,
    pub phases: Vec<SubscriptionPhase>,
}
impl std::fmt::Display for CatalogSubscriptionPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}