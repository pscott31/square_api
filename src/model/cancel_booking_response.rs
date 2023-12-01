
use serde::{Serialize, Deserialize};
use super::{Booking, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CancelBookingResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking: Option<Booking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for CancelBookingResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}