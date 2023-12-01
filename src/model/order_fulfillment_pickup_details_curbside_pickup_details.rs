
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderFulfillmentPickupDetailsCurbsidePickupDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_arrived_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curbside_details: Option<String>,
}
impl std::fmt::Display for OrderFulfillmentPickupDetailsCurbsidePickupDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}