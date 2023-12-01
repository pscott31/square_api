
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AcceptedPaymentMethods {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_app_pay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<bool>,
}
impl std::fmt::Display for AcceptedPaymentMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}