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
pub struct WritableTenant {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "group", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group: Option<Option<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
    #[serde(rename = "circuit_count", skip_serializing_if = "Option::is_none")]
    pub circuit_count: Option<i32>,
    #[serde(rename = "device_count", skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
    #[serde(rename = "ipaddress_count", skip_serializing_if = "Option::is_none")]
    pub ipaddress_count: Option<i32>,
    #[serde(rename = "prefix_count", skip_serializing_if = "Option::is_none")]
    pub prefix_count: Option<i32>,
    #[serde(rename = "rack_count", skip_serializing_if = "Option::is_none")]
    pub rack_count: Option<i32>,
    #[serde(rename = "site_count", skip_serializing_if = "Option::is_none")]
    pub site_count: Option<i32>,
    #[serde(rename = "virtualmachine_count", skip_serializing_if = "Option::is_none")]
    pub virtualmachine_count: Option<i32>,
    #[serde(rename = "vlan_count", skip_serializing_if = "Option::is_none")]
    pub vlan_count: Option<i32>,
    #[serde(rename = "vrf_count", skip_serializing_if = "Option::is_none")]
    pub vrf_count: Option<i32>,
    #[serde(rename = "cluster_count", skip_serializing_if = "Option::is_none")]
    pub cluster_count: Option<i32>,
}

impl WritableTenant {
    pub fn new(name: String, slug: String) -> WritableTenant {
        WritableTenant {
            id: None,
            url: None,
            display: None,
            name,
            slug,
            group: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            circuit_count: None,
            device_count: None,
            ipaddress_count: None,
            prefix_count: None,
            rack_count: None,
            site_count: None,
            virtualmachine_count: None,
            vlan_count: None,
            vrf_count: None,
            cluster_count: None,
        }
    }
}


