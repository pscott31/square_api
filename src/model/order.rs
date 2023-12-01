
use serde::{Serialize, Deserialize};
use super::{
    Money, OrderFulfillment, OrderLineItem, OrderLineItemDiscount, OrderLineItemTax,
    OrderMoneyAmounts, OrderPricingOptions, OrderReturn, OrderReward,
    OrderRoundingAdjustment, OrderServiceCharge, OrderSource, Refund, Tender,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Order {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<OrderLineItemDiscount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillments: Option<Vec<OrderFulfillment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<OrderLineItem>>,
    pub location_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount_due_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amounts: Option<OrderMoneyAmounts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_options: Option<OrderPricingOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<Vec<Refund>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_amounts: Option<OrderMoneyAmounts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<Vec<OrderReturn>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<OrderReward>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rounding_adjustment: Option<OrderRoundingAdjustment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_charges: Option<Vec<OrderServiceCharge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<OrderSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<OrderLineItemTax>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenders: Option<Vec<Tender>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_service_charge_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tip_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}