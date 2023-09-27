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
pub struct Job {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "object_type")]
    pub object_type: String,
    #[serde(rename = "object_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<Option<i64>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status")]
    pub status: Box<crate::models::JobStatus>,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "scheduled", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<Option<String>>,
    /// Recurrence interval (in minutes)
    #[serde(rename = "interval", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub interval: Option<Option<i32>>,
    #[serde(rename = "started", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub started: Option<Option<String>>,
    #[serde(rename = "completed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub completed: Option<Option<String>>,
    #[serde(rename = "user")]
    pub user: Box<crate::models::NestedUser>,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<::std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "job_id")]
    pub job_id: uuid::Uuid,
}

impl Job {
    pub fn new(id: i32, url: String, display: String, object_type: String, name: String, status: crate::models::JobStatus, created: String, user: crate::models::NestedUser, job_id: uuid::Uuid) -> Job {
        Job {
            id,
            url,
            display,
            object_type,
            object_id: None,
            name,
            status: Box::new(status),
            created,
            scheduled: None,
            interval: None,
            started: None,
            completed: None,
            user: Box::new(user),
            data: None,
            job_id,
        }
    }
}

