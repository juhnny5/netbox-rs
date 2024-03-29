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
pub struct NestedManufacturer {
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
    #[serde(rename = "devicetype_count", skip_serializing_if = "Option::is_none")]
    pub devicetype_count: Option<i32>,
}

impl NestedManufacturer {
    pub fn new(name: String, slug: String) -> NestedManufacturer {
        NestedManufacturer {
            id: None,
            url: None,
            display: None,
            name,
            slug,
            devicetype_count: None,
        }
    }
}


