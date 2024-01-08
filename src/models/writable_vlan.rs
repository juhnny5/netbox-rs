/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 3.4
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableVlan {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "site", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub site: Option<Option<i32>>,
    #[serde(rename = "group", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group: Option<Option<i32>>,
    #[serde(rename = "vid")]
    pub vid: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "l2vpn_termination", skip_serializing_if = "Option::is_none")]
    pub l2vpn_termination: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
    #[serde(rename = "prefix_count", skip_serializing_if = "Option::is_none")]
    pub prefix_count: Option<i32>,
}

impl WritableVlan {
    pub fn new(vid: i32, name: String) -> WritableVlan {
        WritableVlan {
            id: None,
            url: None,
            display: None,
            site: None,
            group: None,
            vid,
            name,
            tenant: None,
            status: None,
            role: None,
            description: None,
            comments: None,
            l2vpn_termination: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            prefix_count: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "deprecated")]
    Deprecated,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
