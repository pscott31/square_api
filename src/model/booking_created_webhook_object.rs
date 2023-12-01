
use serde::{Serialize, Deserialize};
use super::Booking;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BookingCreatedWebhookObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking: Option<Booking>,
}
impl std::fmt::Display for BookingCreatedWebhookObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}