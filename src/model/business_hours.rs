
use serde::{Serialize, Deserialize};
use super::BusinessHoursPeriod;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BusinessHours {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periods: Option<Vec<BusinessHoursPeriod>>,
}
impl std::fmt::Display for BusinessHours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}