
use serde::{Serialize, Deserialize};
use super::{Error, Snippet};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpsertSnippetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<Snippet>,
}
impl std::fmt::Display for UpsertSnippetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}