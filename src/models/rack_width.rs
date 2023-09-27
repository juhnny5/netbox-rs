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
pub struct RackWidth {
    /// * `10` - 10 inches * `19` - 19 inches * `21` - 21 inches * `23` - 23 inches
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl RackWidth {
    pub fn new() -> RackWidth {
        RackWidth {
            value: None,
            label: None,
        }
    }
}

/// * `10` - 10 inches * `19` - 19 inches * `21` - 21 inches * `23` - 23 inches
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "10")]
    Variant10,
    #[serde(rename = "19")]
    Variant19,
    #[serde(rename = "21")]
    Variant21,
    #[serde(rename = "23")]
    Variant23,
}

impl Default for Value {
    fn default() -> Value {
        Self::Variant10
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "10 inches")]
    Variant10Inches,
    #[serde(rename = "19 inches")]
    Variant19Inches,
    #[serde(rename = "21 inches")]
    Variant21Inches,
    #[serde(rename = "23 inches")]
    Variant23Inches,
}

impl Default for Label {
    fn default() -> Label {
        Self::Variant10Inches
    }
}

