
use serde::{Serialize, Deserialize};
use super::{DisputeEvidence, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListDisputeEvidenceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Vec<DisputeEvidence>>,
}
impl std::fmt::Display for ListDisputeEvidenceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}