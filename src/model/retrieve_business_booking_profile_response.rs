
use serde::{Serialize, Deserialize};
use super::{BusinessBookingProfile, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrieveBusinessBookingProfileResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_booking_profile: Option<BusinessBookingProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for RetrieveBusinessBookingProfileResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}