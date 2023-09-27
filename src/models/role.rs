/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Role : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
    #[serde(rename = "prefix_count")]
    pub prefix_count: i32,
    #[serde(rename = "vlan_count")]
    pub vlan_count: i32,
}

impl Role {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, name: String, slug: String, created: Option<String>, last_updated: Option<String>, prefix_count: i32, vlan_count: i32) -> Role {
        Role {
            id,
            url,
            display,
            name,
            slug,
            weight: None,
            description: None,
            tags: None,
            custom_fields: None,
            created,
            last_updated,
            prefix_count,
            vlan_count,
        }
    }
}


