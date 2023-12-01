
use serde::{Serialize, Deserialize};
use super::{CashDrawerShift, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveCashDrawerShiftResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_drawer_shift: Option<CashDrawerShift>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for RetrieveCashDrawerShiftResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}