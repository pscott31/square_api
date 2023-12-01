
use serde::{Serialize, Deserialize};
use super::{Error, PaymentLink};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrievePaymentLinkResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<PaymentLink>,
}
impl std::fmt::Display for RetrievePaymentLinkResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}