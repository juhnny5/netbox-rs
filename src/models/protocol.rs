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
pub struct Protocol {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Protocol {
    pub fn new(label: Label, value: Value) -> Protocol {
        Protocol {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
    #[serde(rename = "SCTP")]
    Sctp,
}

impl Default for Label {
    fn default() -> Label {
        Self::Tcp
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "sctp")]
    Sctp,
}

impl Default for Value {
    fn default() -> Value {
        Self::Tcp
    }
}

