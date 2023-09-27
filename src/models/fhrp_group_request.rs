/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FhrpGroupRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FhrpGroupRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// * `vrrp2` - VRRPv2 * `vrrp3` - VRRPv3 * `carp` - CARP * `clusterxl` - ClusterXL * `hsrp` - HSRP * `glbp` - GLBP * `other` - Other
    #[serde(rename = "protocol")]
    pub protocol: Protocol,
    #[serde(rename = "group_id")]
    pub group_id: i32,
    /// * `plaintext` - Plaintext * `md5` - MD5
    #[serde(rename = "auth_type", skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthType>,
    #[serde(rename = "auth_key", skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl FhrpGroupRequest {
    /// Adds support for custom fields and tags.
    pub fn new(protocol: Protocol, group_id: i32) -> FhrpGroupRequest {
        FhrpGroupRequest {
            name: None,
            protocol,
            group_id,
            auth_type: None,
            auth_key: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}

/// * `vrrp2` - VRRPv2 * `vrrp3` - VRRPv3 * `carp` - CARP * `clusterxl` - ClusterXL * `hsrp` - HSRP * `glbp` - GLBP * `other` - Other
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "vrrp2")]
    Vrrp2,
    #[serde(rename = "vrrp3")]
    Vrrp3,
    #[serde(rename = "carp")]
    Carp,
    #[serde(rename = "clusterxl")]
    Clusterxl,
    #[serde(rename = "hsrp")]
    Hsrp,
    #[serde(rename = "glbp")]
    Glbp,
    #[serde(rename = "other")]
    Other,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Vrrp2
    }
}
/// * `plaintext` - Plaintext * `md5` - MD5
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthType {
    #[serde(rename = "plaintext")]
    Plaintext,
    #[serde(rename = "md5")]
    Md5,
    #[serde(rename = "")]
    Empty,
}

impl Default for AuthType {
    fn default() -> AuthType {
        Self::Plaintext
    }
}

