
use serde::{Serialize, Deserialize};
use super::Money;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BusinessAppointmentSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_team_member_booking_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_fee_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_window_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_appointments_per_day_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_appointments_per_day_limit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_booking_lead_time_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_booking_lead_time_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_service_booking_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_booking_flow_staff_selection: Option<bool>,
}
impl std::fmt::Display for BusinessAppointmentSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}