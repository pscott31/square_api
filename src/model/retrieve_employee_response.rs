
use serde::{Serialize, Deserialize};
use super::{Employee, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveEmployeeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee: Option<Employee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for RetrieveEmployeeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}