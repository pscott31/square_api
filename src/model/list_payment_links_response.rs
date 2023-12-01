
use serde::{Serialize, Deserialize};
use super::{Error, PaymentLink};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListPaymentLinksResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_links: Option<Vec<PaymentLink>>,
}
impl std::fmt::Display for ListPaymentLinksResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}