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
pub struct ObjectChange {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::NestedUser>>,
    #[serde(rename = "user_name", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<uuid::Uuid>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
    #[serde(rename = "changed_object_type", skip_serializing_if = "Option::is_none")]
    pub changed_object_type: Option<String>,
    #[serde(rename = "changed_object_id")]
    pub changed_object_id: i32,
    ///  Serialize a nested representation of the changed object. 
    #[serde(rename = "changed_object", skip_serializing_if = "Option::is_none")]
    pub changed_object: Option<serde_json::Value>,
    #[serde(rename = "prechange_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prechange_data: Option<Option<serde_json::Value>>,
    #[serde(rename = "postchange_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub postchange_data: Option<Option<serde_json::Value>>,
}

impl ObjectChange {
    pub fn new(changed_object_id: i32) -> ObjectChange {
        ObjectChange {
            id: None,
            url: None,
            display: None,
            time: None,
            user: None,
            user_name: None,
            request_id: None,
            action: None,
            changed_object_type: None,
            changed_object_id,
            changed_object: None,
            prechange_data: None,
            postchange_data: None,
        }
    }
}


