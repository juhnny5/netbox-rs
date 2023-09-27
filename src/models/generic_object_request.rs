/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GenericObjectRequest : Minimal representation of some generic object identified by ContentType and PK.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericObjectRequest {
    #[serde(rename = "object_type")]
    pub object_type: String,
    #[serde(rename = "object_id")]
    pub object_id: i32,
}

impl GenericObjectRequest {
    /// Minimal representation of some generic object identified by ContentType and PK.
    pub fn new(object_type: String, object_id: i32) -> GenericObjectRequest {
        GenericObjectRequest {
            object_type,
            object_id,
        }
    }
}


