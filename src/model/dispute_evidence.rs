
use serde::{Serialize, Deserialize};
use super::DisputeEvidenceFile;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputeEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_file: Option<DisputeEvidenceFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<String>,
}
impl std::fmt::Display for DisputeEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}