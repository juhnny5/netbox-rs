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
pub struct Type7 {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Type7 {
    pub fn new(label: Label, value: Value) -> Type7 {
        Type7 {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Text")]
    Text,
    #[serde(rename = "Text (long)")]
    TextLeftParenthesisLongRightParenthesis,
    #[serde(rename = "Integer")]
    Integer,
    #[serde(rename = "Decimal")]
    Decimal,
    #[serde(rename = "Boolean (true/false)")]
    BooleanLeftParenthesisTrueSlashFalseRightParenthesis,
    #[serde(rename = "Date")]
    Date,
    #[serde(rename = "URL")]
    Url,
    #[serde(rename = "JSON")]
    Json,
    #[serde(rename = "Selection")]
    Selection,
    #[serde(rename = "Multiple selection")]
    MultipleSelection,
    #[serde(rename = "Object")]
    Object,
    #[serde(rename = "Multiple objects")]
    MultipleObjects,
}

impl Default for Label {
    fn default() -> Label {
        Self::Text
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "longtext")]
    Longtext,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "multiselect")]
    Multiselect,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "multiobject")]
    Multiobject,
}

impl Default for Value {
    fn default() -> Value {
        Self::Text
    }
}

