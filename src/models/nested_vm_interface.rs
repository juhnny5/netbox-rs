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
pub struct NestedVmInterface {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "virtual_machine", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<Option<Box<crate::models::NestedVirtualMachine>>>,
    #[serde(rename = "name")]
    pub name: String,
}

impl NestedVmInterface {
    pub fn new(name: String) -> NestedVmInterface {
        NestedVmInterface {
            id: None,
            url: None,
            display: None,
            virtual_machine: None,
            name,
        }
    }
}


