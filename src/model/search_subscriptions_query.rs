
use serde::{Serialize, Deserialize};
use super::SearchSubscriptionsFilter;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchSubscriptionsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<SearchSubscriptionsFilter>,
}
impl std::fmt::Display for SearchSubscriptionsQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}