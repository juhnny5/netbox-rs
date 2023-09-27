/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VlanGroupRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VlanGroupRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "scope_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<Option<String>>,
    #[serde(rename = "scope_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<Option<i32>>,
    /// Lowest permissible ID of a child VLAN
    #[serde(rename = "min_vid", skip_serializing_if = "Option::is_none")]
    pub min_vid: Option<i32>,
    /// Highest permissible ID of a child VLAN
    #[serde(rename = "max_vid", skip_serializing_if = "Option::is_none")]
    pub max_vid: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl VlanGroupRequest {
    /// Adds support for custom fields and tags.
    pub fn new(name: String, slug: String) -> VlanGroupRequest {
        VlanGroupRequest {
            name,
            slug,
            scope_type: None,
            scope_id: None,
            min_vid: None,
            max_vid: None,
            description: None,
            tags: None,
            custom_fields: None,
        }
    }
}


