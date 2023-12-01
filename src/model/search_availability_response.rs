
use serde::{Serialize, Deserialize};
use super::{Availability, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchAvailabilityResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availabilities: Option<Vec<Availability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for SearchAvailabilityResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}