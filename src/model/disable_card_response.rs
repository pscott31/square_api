
use serde::{Serialize, Deserialize};
use super::{Card, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for DisableCardResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}