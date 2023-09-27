/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DataSource : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: Box<crate::models::DataSourceType>,
    #[serde(rename = "source_url")]
    pub source_url: String,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "status")]
    pub status: Box<crate::models::DataSourceStatus>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "parameters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Option<::std::collections::HashMap<String, serde_json::Value>>>,
    /// Patterns (one per line) matching files to ignore when syncing
    #[serde(rename = "ignore_rules", skip_serializing_if = "Option::is_none")]
    pub ignore_rules: Option<String>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
    #[serde(rename = "file_count")]
    pub file_count: i32,
}

impl DataSource {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, name: String, r#type: crate::models::DataSourceType, source_url: String, status: crate::models::DataSourceStatus, created: Option<String>, last_updated: Option<String>, file_count: i32) -> DataSource {
        DataSource {
            id,
            url,
            display,
            name,
            r#type: Box::new(r#type),
            source_url,
            enabled: None,
            status: Box::new(status),
            description: None,
            comments: None,
            parameters: None,
            ignore_rules: None,
            created,
            last_updated,
            file_count,
        }
    }
}

