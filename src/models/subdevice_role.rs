/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 3.4
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubdeviceRole {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl SubdeviceRole {
    pub fn new(label: Label, value: Value) -> SubdeviceRole {
        SubdeviceRole {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Parent")]
    Parent,
    #[serde(rename = "Child")]
    Child,
}

impl Default for Label {
    fn default() -> Label {
        Self::Parent
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "child")]
    Child,
}

impl Default for Value {
    fn default() -> Value {
        Self::Parent
    }
}

