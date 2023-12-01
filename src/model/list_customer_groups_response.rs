
use serde::{Serialize, Deserialize};
use super::{CustomerGroup, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListCustomerGroupsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<CustomerGroup>>,
}
impl std::fmt::Display for ListCustomerGroupsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}