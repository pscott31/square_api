
use serde::{Serialize, Deserialize};
use super::TipSettings;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceCheckoutOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect_signature: Option<bool>,
    pub device_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_itemized_cart: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_receipt_screen: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_settings: Option<TipSettings>,
}
impl std::fmt::Display for DeviceCheckoutOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}