
use serde::{Serialize, Deserialize};
use super::{LoyaltyAccountExpiringPointDeadline, LoyaltyAccountMapping};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoyaltyAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrolled_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiring_point_deadlines: Option<Vec<LoyaltyAccountExpiringPointDeadline>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime_points: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<LoyaltyAccountMapping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<LoyaltyAccountMapping>>,
    pub program_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl std::fmt::Display for LoyaltyAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}