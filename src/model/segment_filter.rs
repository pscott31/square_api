
use serde::{Serialize, Deserialize};
use super::FilterValue;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SegmentFilter {
    pub service_variation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_id_filter: Option<FilterValue>,
}
impl std::fmt::Display for SegmentFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}