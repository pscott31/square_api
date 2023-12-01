
use serde::{Serialize, Deserialize};
use super::{
    CustomerAddressFilter, CustomerTextFilter, FilterValue, FloatNumberRange, TimeRange,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerCustomAttributeFilterValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CustomerAddressFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<CustomerTextFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<FloatNumberRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<CustomerTextFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<FilterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<CustomerTextFilter>,
}
impl std::fmt::Display for CustomerCustomAttributeFilterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}