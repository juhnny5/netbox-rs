/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JournalEntry : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JournalEntry {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "assigned_object_type")]
    pub assigned_object_type: String,
    #[serde(rename = "assigned_object_id")]
    pub assigned_object_id: i64,
    #[serde(rename = "assigned_object", deserialize_with = "Option::deserialize")]
    pub assigned_object: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "created_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Option<i32>>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<Box<crate::models::JournalEntryKind>>,
    #[serde(rename = "comments")]
    pub comments: String,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
}

impl JournalEntry {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, assigned_object_type: String, assigned_object_id: i64, assigned_object: Option<::std::collections::HashMap<String, serde_json::Value>>, created: Option<String>, comments: String, last_updated: Option<String>) -> JournalEntry {
        JournalEntry {
            id,
            url,
            display,
            assigned_object_type,
            assigned_object_id,
            assigned_object,
            created,
            created_by: None,
            kind: None,
            comments,
            tags: None,
            custom_fields: None,
            last_updated,
        }
    }
}


