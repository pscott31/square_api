
use serde::{Serialize, Deserialize};
use super::{Error, PaymentLink, PaymentLinkRelatedResources};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePaymentLinkResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<PaymentLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resources: Option<PaymentLinkRelatedResources>,
}
impl std::fmt::Display for CreatePaymentLinkResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}