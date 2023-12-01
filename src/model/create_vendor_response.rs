
use serde::{Serialize, Deserialize};
use super::{Error, Vendor};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateVendorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<Vendor>,
}
impl std::fmt::Display for CreateVendorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}