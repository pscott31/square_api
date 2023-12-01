
use serde::{Serialize, Deserialize};
use super::{InventoryAdjustmentGroup, Money, SourceApplication};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InventoryAdjustment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_group: Option<InventoryAdjustmentGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_object_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_receipt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurred_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceApplication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}
impl std::fmt::Display for InventoryAdjustment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}