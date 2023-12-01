
use serde::{Serialize, Deserialize};
use super::{CustomerFilter, CustomerSort};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<CustomerFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<CustomerSort>,
}
impl std::fmt::Display for CustomerQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}