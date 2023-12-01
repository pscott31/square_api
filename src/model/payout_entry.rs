
use serde::{Serialize, Deserialize};
use super::{
    Money, PaymentBalanceActivityAutomaticSavingsDetail,
    PaymentBalanceActivityAutomaticSavingsReversedDetail,
    PaymentBalanceActivityChargeDetail, PaymentBalanceActivityDepositFeeDetail,
    PaymentBalanceActivityDisputeDetail, PaymentBalanceActivityFeeDetail,
    PaymentBalanceActivityFreeProcessingDetail,
    PaymentBalanceActivityHoldAdjustmentDetail, PaymentBalanceActivityOpenDisputeDetail,
    PaymentBalanceActivityOtherAdjustmentDetail, PaymentBalanceActivityOtherDetail,
    PaymentBalanceActivityRefundDetail, PaymentBalanceActivityReleaseAdjustmentDetail,
    PaymentBalanceActivityReserveHoldDetail, PaymentBalanceActivityReserveReleaseDetail,
    PaymentBalanceActivitySquareCapitalPaymentDetail,
    PaymentBalanceActivitySquareCapitalReversedPaymentDetail,
    PaymentBalanceActivityTaxOnFeeDetail, PaymentBalanceActivityThirdPartyFeeDetail,
    PaymentBalanceActivityThirdPartyFeeRefundDetail,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayoutEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount_money: Option<Money>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount_money: Option<Money>,
    pub payout_id: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_automatic_savings_details: Option<
        PaymentBalanceActivityAutomaticSavingsDetail,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_automatic_savings_reversed_details: Option<
        PaymentBalanceActivityAutomaticSavingsReversedDetail,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_charge_details: Option<PaymentBalanceActivityChargeDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_deposit_fee_details: Option<PaymentBalanceActivityDepositFeeDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_dispute_details: Option<PaymentBalanceActivityDisputeDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_fee_details: Option<PaymentBalanceActivityFeeDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_free_processing_details: Option<PaymentBalanceActivityFreeProcessingDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_hold_adjustment_details: Option<PaymentBalanceActivityHoldAdjustmentDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_open_dispute_details: Option<PaymentBalanceActivityOpenDisputeDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_other_adjustment_details: Option<
        PaymentBalanceActivityOtherAdjustmentDetail,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_other_details: Option<PaymentBalanceActivityOtherDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_refund_details: Option<PaymentBalanceActivityRefundDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_release_adjustment_details: Option<
        PaymentBalanceActivityReleaseAdjustmentDetail,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_reserve_hold_details: Option<PaymentBalanceActivityReserveHoldDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_reserve_release_details: Option<PaymentBalanceActivityReserveReleaseDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_square_capital_payment_details: Option<
        PaymentBalanceActivitySquareCapitalPaymentDetail,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_square_capital_reversed_payment_details: Option<
        PaymentBalanceActivitySquareCapitalReversedPaymentDetail,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_tax_on_fee_details: Option<PaymentBalanceActivityTaxOnFeeDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_third_party_fee_details: Option<PaymentBalanceActivityThirdPartyFeeDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_third_party_fee_refund_details: Option<
        PaymentBalanceActivityThirdPartyFeeRefundDetail,
    >,
}
impl std::fmt::Display for PayoutEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}