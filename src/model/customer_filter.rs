
use serde::{Serialize, Deserialize};
use super::{
    CustomerCreationSourceFilter, CustomerCustomAttributeFilters, CustomerTextFilter,
    FilterValue, TimeRange,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<TimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_source: Option<CustomerCreationSourceFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attribute: Option<CustomerCustomAttributeFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<CustomerTextFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<FilterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<CustomerTextFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<CustomerTextFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<TimeRange>,
}
impl std::fmt::Display for CustomerFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}