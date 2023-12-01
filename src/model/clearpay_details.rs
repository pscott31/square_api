
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearpayDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}
impl std::fmt::Display for ClearpayDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}