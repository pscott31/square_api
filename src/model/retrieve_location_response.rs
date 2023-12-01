
use serde::{Serialize, Deserialize};
use super::{Error, Location};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveLocationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}
impl std::fmt::Display for RetrieveLocationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}