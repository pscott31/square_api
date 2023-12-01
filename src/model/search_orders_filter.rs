
use serde::{Serialize, Deserialize};
use super::{
    SearchOrdersCustomerFilter, SearchOrdersDateTimeFilter,
    SearchOrdersFulfillmentFilter, SearchOrdersSourceFilter, SearchOrdersStateFilter,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchOrdersFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_filter: Option<SearchOrdersCustomerFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_filter: Option<SearchOrdersDateTimeFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_filter: Option<SearchOrdersFulfillmentFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_filter: Option<SearchOrdersSourceFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_filter: Option<SearchOrdersStateFilter>,
}
impl std::fmt::Display for SearchOrdersFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}