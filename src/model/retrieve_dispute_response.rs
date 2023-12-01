
use serde::{Serialize, Deserialize};
use super::{Dispute, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveDisputeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute: Option<Dispute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for RetrieveDisputeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}