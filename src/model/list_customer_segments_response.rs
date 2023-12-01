
use serde::{Serialize, Deserialize};
use super::{CustomerSegment, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListCustomerSegmentsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<CustomerSegment>>,
}
impl std::fmt::Display for ListCustomerSegmentsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}