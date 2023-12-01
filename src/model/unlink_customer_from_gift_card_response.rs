
use serde::{Serialize, Deserialize};
use super::{Error, GiftCard};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnlinkCustomerFromGiftCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card: Option<GiftCard>,
}
impl std::fmt::Display for UnlinkCustomerFromGiftCardResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}