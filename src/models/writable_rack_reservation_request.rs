/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WritableRackReservationRequest : Adds support for custom fields and tags.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableRackReservationRequest {
    #[serde(rename = "rack")]
    pub rack: i32,
    #[serde(rename = "units")]
    pub units: Vec<i32>,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTagRequest>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl WritableRackReservationRequest {
    /// Adds support for custom fields and tags.
    pub fn new(rack: i32, units: Vec<i32>, user: i32, description: String) -> WritableRackReservationRequest {
        WritableRackReservationRequest {
            rack,
            units,
            user,
            tenant: None,
            description,
            comments: None,
            tags: None,
            custom_fields: None,
        }
    }
}


