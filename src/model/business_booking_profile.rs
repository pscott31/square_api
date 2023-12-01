
use serde::{Serialize, Deserialize};
use super::BusinessAppointmentSettings;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BusinessBookingProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_cancel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_appointment_settings: Option<BusinessAppointmentSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_timezone_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_seller_level_writes: Option<bool>,
}
impl std::fmt::Display for BusinessBookingProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}