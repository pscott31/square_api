
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JobAssignment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_rate: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_rate: Option<Money>,
    pub job_title: String,
    pub pay_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_hours: Option<i64>,
}
impl std::fmt::Display for JobAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}