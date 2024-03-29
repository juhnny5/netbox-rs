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
pub struct NestedVirtualDeviceContext {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    /// Numeric identifier unique to the parent device
    #[serde(rename = "identifier", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Option<i32>>,
    #[serde(rename = "device")]
    pub device: Box<crate::models::NestedDevice>,
}

impl NestedVirtualDeviceContext {
    pub fn new(name: String, device: crate::models::NestedDevice) -> NestedVirtualDeviceContext {
        NestedVirtualDeviceContext {
            id: None,
            url: None,
            display: None,
            name,
            identifier: None,
            device: Box::new(device),
        }
    }
}


