/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WritableAsnRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableAsnRequest {
    /// 16- or 32-bit autonomous system number
    #[serde(rename = "asn")]
    pub asn: i64,
    /// Regional Internet Registry responsible for this AS number space
    #[serde(rename = "rir")]
    pub rir: i32,
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

impl WritableAsnRequest {
    /// Adds support for custom fields and tags.
    pub fn new(asn: i64, rir: i32) -> WritableAsnRequest {
        WritableAsnRequest {
            asn,
            rir,
            tenant: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}

