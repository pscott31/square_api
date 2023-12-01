
use serde::{Serialize, Deserialize};
use super::{Error, WorkweekConfig};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateWorkweekConfigResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workweek_config: Option<WorkweekConfig>,
}
impl std::fmt::Display for UpdateWorkweekConfigResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}