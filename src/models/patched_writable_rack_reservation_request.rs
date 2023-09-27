/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedWritableRackReservationRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWritableRackReservationRequest {
    #[serde(rename = "rack", skip_serializing_if = "Option::is_none")]
    pub rack: Option<i32>,
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<Vec<i32>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<i32>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedWritableRackReservationRequest {
    /// Adds support for custom fields and tags.
    pub fn new() -> PatchedWritableRackReservationRequest {
        PatchedWritableRackReservationRequest {
            rack: None,
            units: None,
            user: None,
            tenant: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}


