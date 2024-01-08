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
pub struct ModuleType {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "manufacturer")]
    pub manufacturer: Box<crate::models::NestedManufacturer>,
    #[serde(rename = "model")]
    pub model: String,
    /// Discrete part number (optional)
    #[serde(rename = "part_number", skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "weight", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub weight: Option<Option<f32>>,
    #[serde(rename = "weight_unit", skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<Box<crate::models::WeightUnit>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
}

impl ModuleType {
    pub fn new(manufacturer: crate::models::NestedManufacturer, model: String) -> ModuleType {
        ModuleType {
            id: None,
            url: None,
            display: None,
            manufacturer: Box::new(manufacturer),
            model,
            part_number: None,
            weight: None,
            weight_unit: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
        }
    }
}


