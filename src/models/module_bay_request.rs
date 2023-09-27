/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ModuleBayRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleBayRequest {
    #[serde(rename = "device")]
    pub device: Box<crate::models::NestedDeviceRequest>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "installed_module", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub installed_module: Option<Option<Box<crate::models::ModuleBayNestedModuleRequest>>>,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Identifier to reference when renaming installed components
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl ModuleBayRequest {
    /// Adds support for custom fields and tags.
    pub fn new(device: crate::models::NestedDeviceRequest, name: String) -> ModuleBayRequest {
        ModuleBayRequest {
            device: Box::new(device),
            name,
            installed_module: None,
            label: None,
            position: None,
            description: None,
            tags: None,
            custom_fields: None,
        }
    }
}

