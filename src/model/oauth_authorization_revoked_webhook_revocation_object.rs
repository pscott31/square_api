
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OauthAuthorizationRevokedWebhookRevocationObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoker_type: Option<String>,
}
impl std::fmt::Display for OauthAuthorizationRevokedWebhookRevocationObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}