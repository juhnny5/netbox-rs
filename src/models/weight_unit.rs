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
pub struct WeightUnit {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl WeightUnit {
    pub fn new(label: Label, value: Value) -> WeightUnit {
        WeightUnit {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Kilograms")]
    Kilograms,
    #[serde(rename = "Grams")]
    Grams,
    #[serde(rename = "Pounds")]
    Pounds,
    #[serde(rename = "Ounces")]
    Ounces,
}

impl Default for Label {
    fn default() -> Label {
        Self::Kilograms
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "kg")]
    Kg,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "oz")]
    Oz,
}

impl Default for Value {
    fn default() -> Value {
        Self::Kg
    }
}

