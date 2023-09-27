/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConfigContextRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigContextRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<i32>>,
    #[serde(rename = "site_groups", skip_serializing_if = "Option::is_none")]
    pub site_groups: Option<Vec<i32>>,
    #[serde(rename = "sites", skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<i32>>,
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<i32>>,
    #[serde(rename = "device_types", skip_serializing_if = "Option::is_none")]
    pub device_types: Option<Vec<i32>>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<i32>>,
    #[serde(rename = "platforms", skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<i32>>,
    #[serde(rename = "cluster_types", skip_serializing_if = "Option::is_none")]
    pub cluster_types: Option<Vec<i32>>,
    #[serde(rename = "cluster_groups", skip_serializing_if = "Option::is_none")]
    pub cluster_groups: Option<Vec<i32>>,
    #[serde(rename = "clusters", skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<i32>>,
    #[serde(rename = "tenant_groups", skip_serializing_if = "Option::is_none")]
    pub tenant_groups: Option<Vec<i32>>,
    #[serde(rename = "tenants", skip_serializing_if = "Option::is_none")]
    pub tenants: Option<Vec<i32>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<Box<crate::models::NestedDataSourceRequest>>,
    #[serde(rename = "data")]
    pub data: ::std::collections::HashMap<String, serde_json::Value>,
}

impl ConfigContextRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(name: String, data: ::std::collections::HashMap<String, serde_json::Value>) -> ConfigContextRequest {
        ConfigContextRequest {
            name,
            weight: None,
            description: None,
            is_active: None,
            regions: None,
            site_groups: None,
            sites: None,
            locations: None,
            device_types: None,
            roles: None,
            platforms: None,
            cluster_types: None,
            cluster_groups: None,
            clusters: None,
            tenant_groups: None,
            tenants: None,
            tags: None,
            data_source: None,
            data,
        }
    }
}


