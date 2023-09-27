/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JournalEntryRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JournalEntryRequest {
    #[serde(rename = "assigned_object_type")]
    pub assigned_object_type: String,
    #[serde(rename = "assigned_object_id")]
    pub assigned_object_id: i64,
    #[serde(rename = "created_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Option<i32>>,
    /// * `info` - Info * `success` - Success * `warning` - Warning * `danger` - Danger
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    #[serde(rename = "comments")]
    pub comments: String,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl JournalEntryRequest {
    /// Adds support for custom fields and tags.
    pub fn new(assigned_object_type: String, assigned_object_id: i64, comments: String) -> JournalEntryRequest {
        JournalEntryRequest {
            assigned_object_type,
            assigned_object_id,
            created_by: None,
            kind: None,
            comments,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `info` - Info * `success` - Success * `warning` - Warning * `danger` - Danger
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "danger")]
    Danger,
}

impl Default for Kind {
    fn default() -> Kind {
        Self::Info
    }
}

