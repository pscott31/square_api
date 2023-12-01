
use serde::{Serialize, Deserialize};
use super::DeviceCode;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceCodePairedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_code: Option<DeviceCode>,
}
impl std::fmt::Display for DeviceCodePairedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}