/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PatchedSavedFilterRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedSavedFilterRequest {
    #[serde(rename = "content_types", skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<String>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user: Option<Option<i32>>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PatchedSavedFilterRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new() -> PatchedSavedFilterRequest {
        PatchedSavedFilterRequest {
            content_types: None,
            name: None,
            slug: None,
            description: None,
            user: None,
            weight: None,
            enabled: None,
            shared: None,
            parameters: None,
        }
    }
}

