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
pub struct InventoryItemTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device_type")]
    pub device_type: Box<crate::models::NestedDeviceType>,
    #[serde(rename = "parent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Option<i32>>,
    ///  {module} is accepted as a substitution for the module bay position when attached to a module type. 
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<Box<crate::models::NestedInventoryItemRole>>>,
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<Box<crate::models::NestedManufacturer>>,
    /// Manufacturer-assigned part identifier
    #[serde(rename = "part_id", skip_serializing_if = "Option::is_none")]
    pub part_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "component_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub component_type: Option<Option<String>>,
    #[serde(rename = "component_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub component_id: Option<Option<i32>>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
    #[serde(rename = "_depth", skip_serializing_if = "Option::is_none")]
    pub _depth: Option<i32>,
}

impl InventoryItemTemplate {
    pub fn new(device_type: crate::models::NestedDeviceType, name: String) -> InventoryItemTemplate {
        InventoryItemTemplate {
            id: None,
            url: None,
            display: None,
            device_type: Box::new(device_type),
            parent: None,
            name,
            label: None,
            role: None,
            manufacturer: None,
            part_id: None,
            description: None,
            component_type: None,
            component_id: None,
            component: None,
            created: None,
            last_updated: None,
            _depth: None,
        }
    }
}


