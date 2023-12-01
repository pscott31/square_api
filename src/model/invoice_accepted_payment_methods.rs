
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceAcceptedPaymentMethods {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_gift_card: Option<bool>,
}
impl std::fmt::Display for InvoiceAcceptedPaymentMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}