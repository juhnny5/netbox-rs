/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CircuitRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CircuitRequest {
    /// Unique circuit ID
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "provider")]
    pub provider: Box<crate::models::NestedProviderRequest>,
    #[serde(rename = "provider_account", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub provider_account: Option<Option<Box<crate::models::NestedProviderAccountRequest>>>,
    #[serde(rename = "type")]
    pub r#type: Box<crate::models::NestedCircuitTypeRequest>,
    /// * `planned` - Planned * `provisioning` - Provisioning * `active` - Active * `offline` - Offline * `deprovisioning` - Deprovisioning * `decommissioned` - Decommissioned
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenantRequest>>>,
    #[serde(rename = "install_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub install_date: Option<Option<String>>,
    #[serde(rename = "termination_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<Option<String>>,
    /// Committed rate
    #[serde(rename = "commit_rate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub commit_rate: Option<Option<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl CircuitRequest {
    /// Adds support for custom fields and tags.
    pub fn new(cid: String, provider: crate::models::NestedProviderRequest, r#type: crate::models::NestedCircuitTypeRequest) -> CircuitRequest {
        CircuitRequest {
            cid,
            provider: Box::new(provider),
            provider_account: None,
            r#type: Box::new(r#type),
            status: None,
            tenant: None,
            install_date: None,
            termination_date: None,
            commit_rate: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `planned` - Planned * `provisioning` - Provisioning * `active` - Active * `offline` - Offline * `deprovisioning` - Deprovisioning * `decommissioned` - Decommissioned
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "provisioning")]
    Provisioning,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "deprovisioning")]
    Deprovisioning,
    #[serde(rename = "decommissioned")]
    Decommissioned,
}

impl Default for Status {
    fn default() -> Status {
        Self::Planned
    }
}

