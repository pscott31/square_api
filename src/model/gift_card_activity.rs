
use serde::{Serialize, Deserialize};
use super::{
    GiftCardActivityActivate, GiftCardActivityAdjustDecrement,
    GiftCardActivityAdjustIncrement, GiftCardActivityBlock, GiftCardActivityClearBalance,
    GiftCardActivityDeactivate, GiftCardActivityImport, GiftCardActivityImportReversal,
    GiftCardActivityLoad, GiftCardActivityRedeem, GiftCardActivityRefund,
    GiftCardActivityUnblock, GiftCardActivityUnlinkedActivityRefund, Money,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GiftCardActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate_activity_details: Option<GiftCardActivityActivate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_decrement_activity_details: Option<GiftCardActivityAdjustDecrement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_increment_activity_details: Option<GiftCardActivityAdjustIncrement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_activity_details: Option<GiftCardActivityBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_balance_activity_details: Option<GiftCardActivityClearBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivate_activity_details: Option<GiftCardActivityDeactivate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_balance_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_gan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_activity_details: Option<GiftCardActivityImport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_reversal_activity_details: Option<GiftCardActivityImportReversal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_activity_details: Option<GiftCardActivityLoad>,
    pub location_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_activity_details: Option<GiftCardActivityRedeem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_activity_details: Option<GiftCardActivityRefund>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unblock_activity_details: Option<GiftCardActivityUnblock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlinked_activity_refund_activity_details: Option<
        GiftCardActivityUnlinkedActivityRefund,
    >,
}
impl std::fmt::Display for GiftCardActivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}