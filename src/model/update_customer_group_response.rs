
use serde::{Serialize, Deserialize};
use super::{CustomerGroup, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCustomerGroupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<CustomerGroup>,
}
impl std::fmt::Display for UpdateCustomerGroupResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}