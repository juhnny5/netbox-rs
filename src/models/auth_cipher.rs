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
pub struct AuthCipher {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl AuthCipher {
    pub fn new(label: Label, value: Value) -> AuthCipher {
        AuthCipher {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Auto")]
    Auto,
    #[serde(rename = "TKIP")]
    Tkip,
    #[serde(rename = "AES")]
    Aes,
}

impl Default for Label {
    fn default() -> Label {
        Self::Auto
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "tkip")]
    Tkip,
    #[serde(rename = "aes")]
    Aes,
}

impl Default for Value {
    fn default() -> Value {
        Self::Auto
    }
}

