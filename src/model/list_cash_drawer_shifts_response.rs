
use serde::{Serialize, Deserialize};
use super::{CashDrawerShiftSummary, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListCashDrawerShiftsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CashDrawerShiftSummary>>,
}
impl std::fmt::Display for ListCashDrawerShiftsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}