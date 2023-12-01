
use serde::{Serialize, Deserialize};
use super::SearchAvailabilityFilter;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAvailabilityQuery {
    pub filter: SearchAvailabilityFilter,
}
impl std::fmt::Display for SearchAvailabilityQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}