
use serde::{Serialize, Deserialize};
use super::{EmployeeWage, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetEmployeeWageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_wage: Option<EmployeeWage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for GetEmployeeWageResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}