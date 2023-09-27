/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Module : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Module {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "device")]
    pub device: Box<crate::models::NestedDevice>,
    #[serde(rename = "module_bay")]
    pub module_bay: Box<crate::models::NestedModuleBay>,
    #[serde(rename = "module_type")]
    pub module_type: Box<crate::models::NestedModuleType>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::ModuleStatus>>,
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// A unique tag used to identify this device
    #[serde(rename = "asset_tag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<Option<String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
}

impl Module {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, device: crate::models::NestedDevice, module_bay: crate::models::NestedModuleBay, module_type: crate::models::NestedModuleType, created: Option<String>, last_updated: Option<String>) -> Module {
        Module {
            id,
            url,
            display,
            device: Box::new(device),
            module_bay: Box::new(module_bay),
            module_type: Box::new(module_type),
            status: None,
            serial: None,
            asset_tag: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
        }
    }
}


