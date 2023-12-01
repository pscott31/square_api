
use serde::{Serialize, Deserialize};
use super::Error;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CalculateLoyaltyPointsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_points: Option<i64>,
}
impl std::fmt::Display for CalculateLoyaltyPointsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}