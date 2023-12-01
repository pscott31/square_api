
use serde::{Serialize, Deserialize};
use super::{CatalogInfoResponseLimits, Error, StandardUnitDescriptionGroup};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CatalogInfoResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CatalogInfoResponseLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_unit_description_group: Option<StandardUnitDescriptionGroup>,
}
impl std::fmt::Display for CatalogInfoResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}