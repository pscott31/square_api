
use serde::{Serialize, Deserialize};
use super::{DeviceCode, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListDeviceCodesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_codes: Option<Vec<DeviceCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for ListDeviceCodesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}