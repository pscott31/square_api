
use serde::{Serialize, Deserialize};
use super::{
    LoyaltyProgramAccrualRuleCategoryData, LoyaltyProgramAccrualRuleItemVariationData,
    LoyaltyProgramAccrualRuleSpendData, LoyaltyProgramAccrualRuleVisitData, Money,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyProgramAccrualRule {
    pub accrual_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_data: Option<LoyaltyProgramAccrualRuleCategoryData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_category_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_item_variation_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_variation_data: Option<LoyaltyProgramAccrualRuleItemVariationData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_amount_money: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_data: Option<LoyaltyProgramAccrualRuleSpendData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<LoyaltyProgramAccrualRuleVisitData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_minimum_amount_money: Option<Money>,
}
impl std::fmt::Display for LoyaltyProgramAccrualRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}