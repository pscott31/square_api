
use serde::{Serialize, Deserialize};
use super::OauthAuthorizationRevokedWebhookRevocationObject;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OauthAuthorizationRevokedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation: Option<OauthAuthorizationRevokedWebhookRevocationObject>,
}
impl std::fmt::Display for OauthAuthorizationRevokedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}