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
pub struct DeviceWithConfigContext {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "device_type")]
    pub device_type: Box<crate::models::NestedDeviceType>,
    #[serde(rename = "device_role")]
    pub device_role: Box<crate::models::NestedDeviceRole>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    #[serde(rename = "platform", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Option<Box<crate::models::NestedPlatform>>>,
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// A unique tag used to identify this device
    #[serde(rename = "asset_tag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<Option<String>>,
    #[serde(rename = "site", deserialize_with = "Option::deserialize")]
    pub site: Option<Box<crate::models::NestedSite>>,
    #[serde(rename = "location", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<Box<crate::models::NestedLocation>>>,
    #[serde(rename = "rack", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rack: Option<Option<Box<crate::models::NestedRack>>>,
    #[serde(rename = "position", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub position: Option<Option<f32>>,
    #[serde(rename = "face", skip_serializing_if = "Option::is_none")]
    pub face: Option<Box<crate::models::Face>>,
    #[serde(rename = "parent_device", skip_serializing_if = "Option::is_none")]
    pub parent_device: Option<Box<crate::models::NestedDevice>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::Status2>>,
    #[serde(rename = "airflow", skip_serializing_if = "Option::is_none")]
    pub airflow: Option<Box<crate::models::Airflow>>,
    #[serde(rename = "primary_ip", skip_serializing_if = "Option::is_none")]
    pub primary_ip: Option<Box<crate::models::NestedIpAddress>>,
    #[serde(rename = "primary_ip4", skip_serializing_if = "Option::is_none")]
    pub primary_ip4: Option<Box<crate::models::NestedIpAddress>>,
    #[serde(rename = "primary_ip6", skip_serializing_if = "Option::is_none")]
    pub primary_ip6: Option<Box<crate::models::NestedIpAddress>>,
    #[serde(rename = "cluster", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Option<Box<crate::models::NestedCluster>>>,
    #[serde(rename = "virtual_chassis", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub virtual_chassis: Option<Option<Box<crate::models::NestedVirtualChassis>>>,
    #[serde(rename = "vc_position", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vc_position: Option<Option<i32>>,
    #[serde(rename = "vc_priority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vc_priority: Option<Option<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "local_context_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub local_context_data: Option<Option<serde_json::Value>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "config_context", skip_serializing_if = "Option::is_none")]
    pub config_context: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
}

impl DeviceWithConfigContext {
    pub fn new(device_type: crate::models::NestedDeviceType, device_role: crate::models::NestedDeviceRole, site: Option<crate::models::NestedSite>) -> DeviceWithConfigContext {
        DeviceWithConfigContext {
            id: None,
            url: None,
            display: None,
            name: None,
            device_type: Box::new(device_type),
            device_role: Box::new(device_role),
            tenant: None,
            platform: None,
            serial: None,
            asset_tag: None,
            site: if let Some(x) = site {Some(Box::new(x))} else {None},
            location: None,
            rack: None,
            position: None,
            face: None,
            parent_device: None,
            status: None,
            airflow: None,
            primary_ip: None,
            primary_ip4: None,
            primary_ip6: None,
            cluster: None,
            virtual_chassis: None,
            vc_position: None,
            vc_priority: None,
            description: None,
            comments: None,
            local_context_data: None,
            tags: None,
            custom_fields: None,
            config_context: None,
            created: None,
            last_updated: None,
        }
    }
}


