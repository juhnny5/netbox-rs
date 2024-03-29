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
pub struct NestedVlan {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "vid")]
    pub vid: i32,
    #[serde(rename = "name")]
    pub name: String,
}

impl NestedVlan {
    pub fn new(vid: i32, name: String) -> NestedVlan {
        NestedVlan {
            id: None,
            url: None,
            display: None,
            vid,
            name,
        }
    }
}


