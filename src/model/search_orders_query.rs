
use serde::{Serialize, Deserialize};
use super::{SearchOrdersFilter, SearchOrdersSort};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchOrdersQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<SearchOrdersFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SearchOrdersSort>,
}
impl std::fmt::Display for SearchOrdersQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}