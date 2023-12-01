
use serde::{Serialize, Deserialize};
use super::{
    OrderMoneyAmounts, OrderReturnDiscount, OrderReturnLineItem,
    OrderReturnServiceCharge, OrderReturnTax, OrderRoundingAdjustment,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderReturn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_amounts: Option<OrderMoneyAmounts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_discounts: Option<Vec<OrderReturnDiscount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_line_items: Option<Vec<OrderReturnLineItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_service_charges: Option<Vec<OrderReturnServiceCharge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_taxes: Option<Vec<OrderReturnTax>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rounding_adjustment: Option<OrderRoundingAdjustment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl std::fmt::Display for OrderReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}