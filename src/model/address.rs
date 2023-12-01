
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Address {
    #[serde(rename = "address_line_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(rename = "address_line_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(rename = "address_line_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    #[serde(rename = "administrative_district_level_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_district_level1: Option<String>,
    #[serde(rename = "administrative_district_level_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_district_level2: Option<String>,
    #[serde(rename = "administrative_district_level_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_district_level3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sublocality: Option<String>,
    #[serde(rename = "sublocality_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sublocality2: Option<String>,
    #[serde(rename = "sublocality_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sublocality3: Option<String>,
}
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}