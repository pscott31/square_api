
use serde::{Serialize, Deserialize};
use super::Error;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BulkCreateVendorsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<serde_json::Value>,
}
impl std::fmt::Display for BulkCreateVendorsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}