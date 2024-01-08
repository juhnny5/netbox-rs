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
pub struct RfRole {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl RfRole {
    pub fn new(label: Label, value: Value) -> RfRole {
        RfRole {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Access point")]
    AccessPoint,
    #[serde(rename = "Station")]
    Station,
}

impl Default for Label {
    fn default() -> Label {
        Self::AccessPoint
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "ap")]
    Ap,
    #[serde(rename = "station")]
    Station,
}

impl Default for Value {
    fn default() -> Value {
        Self::Ap
    }
}

