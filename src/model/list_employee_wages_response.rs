
use serde::{Serialize, Deserialize};
use super::{EmployeeWage, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListEmployeeWagesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_wages: Option<Vec<EmployeeWage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for ListEmployeeWagesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}