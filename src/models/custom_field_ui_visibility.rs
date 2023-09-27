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
pub struct CustomFieldUiVisibility {
    /// * `read-write` - Read/write * `read-only` - Read-only * `hidden` - Hidden * `hidden-ifunset` - Hidden (if unset)
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl CustomFieldUiVisibility {
    pub fn new() -> CustomFieldUiVisibility {
        CustomFieldUiVisibility {
            value: None,
            label: None,
        }
    }
}

/// * `read-write` - Read/write * `read-only` - Read-only * `hidden` - Hidden * `hidden-ifunset` - Hidden (if unset)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "read-write")]
    ReadWrite,
    #[serde(rename = "read-only")]
    ReadOnly,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "hidden-ifunset")]
    HiddenIfunset,
}

impl Default for Value {
    fn default() -> Value {
        Self::ReadWrite
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Read/write")]
    ReadSlashWrite,
    #[serde(rename = "Read-only")]
    ReadOnly,
    #[serde(rename = "Hidden")]
    Hidden,
    #[serde(rename = "Hidden (if unset)")]
    HiddenLeftParenthesisIfUnsetRightParenthesis,
}

impl Default for Label {
    fn default() -> Label {
        Self::ReadSlashWrite
    }
}

