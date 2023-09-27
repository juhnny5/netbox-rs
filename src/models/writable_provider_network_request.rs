/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WritableProviderNetworkRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableProviderNetworkRequest {
    #[serde(rename = "provider")]
    pub provider: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl WritableProviderNetworkRequest {
    /// Adds support for custom fields and tags.
    pub fn new(provider: i32, name: String) -> WritableProviderNetworkRequest {
        WritableProviderNetworkRequest {
            provider,
            name,
            service_id: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}

