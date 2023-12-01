
use serde::{Serialize, Deserialize};
use super::{DeviceCode, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDeviceCodeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_code: Option<DeviceCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for CreateDeviceCodeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}