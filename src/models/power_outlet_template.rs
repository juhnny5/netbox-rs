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
pub struct PowerOutletTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device_type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<Box<crate::models::NestedDeviceType>>,
    #[serde(rename = "module_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module_type: Option<Option<Box<crate::models::NestedModuleType>>>,
    ///  {module} is accepted as a substitution for the module bay position when attached to a module type. 
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<crate::models::Type4>>,
    #[serde(rename = "power_port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub power_port: Option<Option<Box<crate::models::NestedPowerPortTemplate>>>,
    #[serde(rename = "feed_leg", skip_serializing_if = "Option::is_none")]
    pub feed_leg: Option<Box<crate::models::FeedLeg>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
}

impl PowerOutletTemplate {
    pub fn new(name: String) -> PowerOutletTemplate {
        PowerOutletTemplate {
            id: None,
            url: None,
            display: None,
            device_type: None,
            module_type: None,
            name,
            label: None,
            r#type: None,
            power_port: None,
            feed_leg: None,
            description: None,
            created: None,
            last_updated: None,
        }
    }
}


