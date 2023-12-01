
use serde::{Serialize, Deserialize};
use super::{Checkout, Error};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCheckoutResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout: Option<Checkout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}
impl std::fmt::Display for CreateCheckoutResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}