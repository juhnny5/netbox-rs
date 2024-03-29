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
pub struct UsersPermissionsList200Response {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "next", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub next: Option<Option<String>>,
    #[serde(rename = "previous", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub previous: Option<Option<String>>,
    #[serde(rename = "results")]
    pub results: Vec<crate::models::ObjectPermission>,
}

impl UsersPermissionsList200Response {
    pub fn new(count: i32, results: Vec<crate::models::ObjectPermission>) -> UsersPermissionsList200Response {
        UsersPermissionsList200Response {
            count,
            next: None,
            previous: None,
            results,
        }
    }
}


