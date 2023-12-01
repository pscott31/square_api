
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShiftWage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_rate: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl std::fmt::Display for ShiftWage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}