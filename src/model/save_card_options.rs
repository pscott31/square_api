
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SaveCardOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    pub customer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
}
impl std::fmt::Display for SaveCardOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}