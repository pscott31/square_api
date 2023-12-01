
use serde::{Serialize, Deserialize};
use super::{SegmentFilter, TimeRange};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAvailabilityFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_filters: Option<Vec<SegmentFilter>>,
    pub start_at_range: TimeRange,
}
impl std::fmt::Display for SearchAvailabilityFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}