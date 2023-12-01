
use serde::{Serialize, Deserialize};
use super::{Customer, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCustomerResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for CreateCustomerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}