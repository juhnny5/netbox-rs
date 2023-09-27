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
pub struct DeviceFace {
    /// * `front` - Front * `rear` - Rear
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl DeviceFace {
    pub fn new() -> DeviceFace {
        DeviceFace {
            value: None,
            label: None,
        }
    }
}

/// * `front` - Front * `rear` - Rear
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "front")]
    Front,
    #[serde(rename = "rear")]
    Rear,
    #[serde(rename = "")]
    Empty,
}

impl Default for Value {
    fn default() -> Value {
        Self::Front
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Front")]
    Front,
    #[serde(rename = "Rear")]
    Rear,
}

impl Default for Label {
    fn default() -> Label {
        Self::Front
    }
}

