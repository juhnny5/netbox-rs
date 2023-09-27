/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WritableVrfRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableVrfRequest {
    #[serde(rename = "name")]
    pub name: String,
    /// Unique route distinguisher (as defined in RFC 4364)
    #[serde(rename = "rd", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rd: Option<Option<String>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    /// Prevent duplicate prefixes/IP addresses within this VRF
    #[serde(rename = "enforce_unique", skip_serializing_if = "Option::is_none")]
    pub enforce_unique: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "import_targets", skip_serializing_if = "Option::is_none")]
    pub import_targets: Option<Vec<i32>>,
    #[serde(rename = "export_targets", skip_serializing_if = "Option::is_none")]
    pub export_targets: Option<Vec<i32>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl WritableVrfRequest {
    /// Adds support for custom fields and tags.
    pub fn new(name: String) -> WritableVrfRequest {
        WritableVrfRequest {
            name,
            rd: None,
            tenant: None,
            enforce_unique: None,
            description: None,
            comments: None,
            import_targets: None,
            export_targets: None,
            tags: None,
            custom_fields: None,
        }
    }
}


