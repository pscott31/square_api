
use serde::{Serialize, Deserialize};
use super::{AfterpayDetails, ClearpayDetails};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuyNowPayLaterDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_details: Option<AfterpayDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearpay_details: Option<ClearpayDetails>,
}
impl std::fmt::Display for BuyNowPayLaterDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}