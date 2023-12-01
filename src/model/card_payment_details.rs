
use serde::{Serialize, Deserialize};
use super::{Card, CardPaymentTimeline, DeviceDetails, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardPaymentDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_cryptogram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_result_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payment_timeline: Option<CardPaymentTimeline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_details: Option<DeviceDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_requires_card_presence: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_results: Option<String>,
}
impl std::fmt::Display for CardPaymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}