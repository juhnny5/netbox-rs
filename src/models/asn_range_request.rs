/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AsnRangeRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsnRangeRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "rir")]
    pub rir: Box<crate::models::NestedRirRequest>,
    #[serde(rename = "start")]
    pub start: i64,
    #[serde(rename = "end")]
    pub end: i64,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenantRequest>>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl AsnRangeRequest {
    /// Adds support for custom fields and tags.
    pub fn new(name: String, slug: String, rir: crate::models::NestedRirRequest, start: i64, end: i64) -> AsnRangeRequest {
        AsnRangeRequest {
            name,
            slug,
            rir: Box::new(rir),
            start,
            end,
            tenant: None,
            description: None,
            tags: None,
            custom_fields: None,
        }
    }
}


