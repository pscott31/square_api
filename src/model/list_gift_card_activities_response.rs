
use serde::{Serialize, Deserialize};
use super::{Error, GiftCardActivity};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListGiftCardActivitiesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_activities: Option<Vec<GiftCardActivity>>,
}
impl std::fmt::Display for ListGiftCardActivitiesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}