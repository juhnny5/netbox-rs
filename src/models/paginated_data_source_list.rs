/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedDataSourceList {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "next", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub next: Option<Option<String>>,
    #[serde(rename = "previous", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub previous: Option<Option<String>>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::DataSource>>,
}

impl PaginatedDataSourceList {
    pub fn new() -> PaginatedDataSourceList {
        PaginatedDataSourceList {
            count: None,
            next: None,
            previous: None,
            results: None,
        }
    }
}


