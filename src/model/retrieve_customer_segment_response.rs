
use serde::{Serialize, Deserialize};
use super::{CustomerSegment, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveCustomerSegmentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<CustomerSegment>,
}
impl std::fmt::Display for RetrieveCustomerSegmentResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}