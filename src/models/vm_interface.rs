/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// VmInterface : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmInterface {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "virtual_machine")]
    pub virtual_machine: Box<crate::models::NestedVirtualMachine>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "parent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Option<Box<crate::models::NestedVmInterface>>>,
    #[serde(rename = "bridge", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bridge: Option<Option<Box<crate::models::NestedVmInterface>>>,
    #[serde(rename = "mtu", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mtu: Option<Option<i32>>,
    #[serde(rename = "mac_address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<Option<String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Box<crate::models::InterfaceMode>>,
    #[serde(rename = "untagged_vlan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub untagged_vlan: Option<Option<Box<crate::models::NestedVlan>>>,
    #[serde(rename = "tagged_vlans", skip_serializing_if = "Option::is_none")]
    pub tagged_vlans: Option<Vec<i32>>,
    #[serde(rename = "vrf", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vrf: Option<Option<Box<crate::models::NestedVrf>>>,
    #[serde(rename = "l2vpn_termination", deserialize_with = "Option::deserialize")]
    pub l2vpn_termination: Option<Box<crate::models::NestedL2VpnTermination>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "created", deserialize_with = "Option::deserialize")]
    pub created: Option<String>,
    #[serde(rename = "last_updated", deserialize_with = "Option::deserialize")]
    pub last_updated: Option<String>,
    #[serde(rename = "count_ipaddresses")]
    pub count_ipaddresses: i32,
    #[serde(rename = "count_fhrp_groups")]
    pub count_fhrp_groups: i32,
}

impl VmInterface {
    /// Adds support for custom fields and tags.
    pub fn new(id: i32, url: String, display: String, virtual_machine: crate::models::NestedVirtualMachine, name: String, l2vpn_termination: Option<crate::models::NestedL2VpnTermination>, created: Option<String>, last_updated: Option<String>, count_ipaddresses: i32, count_fhrp_groups: i32) -> VmInterface {
        VmInterface {
            id,
            url,
            display,
            virtual_machine: Box::new(virtual_machine),
            name,
            enabled: None,
            parent: None,
            bridge: None,
            mtu: None,
            mac_address: None,
            description: None,
            mode: None,
            untagged_vlan: None,
            tagged_vlans: None,
            vrf: None,
            l2vpn_termination: if let Some(x) = l2vpn_termination {Some(Box::new(x))} else {None},
            tags: None,
            custom_fields: None,
            created,
            last_updated,
            count_ipaddresses,
            count_fhrp_groups,
        }
    }
}

