
use serde::{Serialize, Deserialize};
use super::{Error, Payment};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePaymentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<Payment>,
}
impl std::fmt::Display for CreatePaymentResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}