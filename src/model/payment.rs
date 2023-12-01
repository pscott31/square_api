
use serde::{Serialize, Deserialize};
use super::{
    Address, ApplicationDetails, BankAccountPaymentDetails, BuyNowPayLaterDetails,
    CardPaymentDetails, CashPaymentDetails, DeviceDetails, DigitalWalletDetails,
    ExternalPaymentDetails, Money, ProcessingFee, RiskEvaluation,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Payment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_fee_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_details: Option<ApplicationDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_details: Option<BankAccountPaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_now_pay_later_details: Option<BuyNowPayLaterDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_details: Option<CardPaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_details: Option<CashPaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delayed_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_details: Option<DeviceDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_details: Option<ExternalPaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_fee: Option<Vec<ProcessingFee>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_evaluation: Option<RiskEvaluation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_description_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_details: Option<DigitalWalletDetails>,
}
impl std::fmt::Display for Payment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}