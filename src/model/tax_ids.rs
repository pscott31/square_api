
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_nif: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_vat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_naf: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_siret: Option<String>,
}
impl std::fmt::Display for TaxIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}