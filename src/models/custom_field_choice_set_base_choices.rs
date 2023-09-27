/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldChoiceSetBaseChoices {
    /// * `IATA` - IATA (Airport codes) * `ISO_3166` - ISO 3166 (Country codes) * `UN_LOCODE` - UN/LOCODE (Location codes)
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl CustomFieldChoiceSetBaseChoices {
    pub fn new() -> CustomFieldChoiceSetBaseChoices {
        CustomFieldChoiceSetBaseChoices {
            value: None,
            label: None,
        }
    }
}

/// * `IATA` - IATA (Airport codes) * `ISO_3166` - ISO 3166 (Country codes) * `UN_LOCODE` - UN/LOCODE (Location codes)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "IATA")]
    Iata,
    #[serde(rename = "ISO_3166")]
    Iso3166,
    #[serde(rename = "UN_LOCODE")]
    UnLocode,
}

impl Default for Value {
    fn default() -> Value {
        Self::Iata
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "IATA (Airport codes)")]
    IataLeftParenthesisAirportCodesRightParenthesis,
    #[serde(rename = "ISO 3166 (Country codes)")]
    Iso3166LeftParenthesisCountryCodesRightParenthesis,
    #[serde(rename = "UN/LOCODE (Location codes)")]
    UnSlashLocodeLeftParenthesisLocationCodesRightParenthesis,
}

impl Default for Label {
    fn default() -> Label {
        Self::IataLeftParenthesisAirportCodesRightParenthesis
    }
}

