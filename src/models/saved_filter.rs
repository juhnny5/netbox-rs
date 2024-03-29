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
pub struct SavedFilter {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "content_types")]
    pub content_types: Vec<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "user", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user: Option<Option<i32>>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(rename = "parameters")]
    pub parameters: serde_json::Value,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
}

impl SavedFilter {
    pub fn new(content_types: Vec<String>, name: String, slug: String, parameters: serde_json::Value) -> SavedFilter {
        SavedFilter {
            id: None,
            url: None,
            display: None,
            content_types,
            name,
            slug,
            description: None,
            user: None,
            weight: None,
            enabled: None,
            shared: None,
            parameters,
            created: None,
            last_updated: None,
        }
    }
}


