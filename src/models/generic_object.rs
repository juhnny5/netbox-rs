/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GenericObject : Minimal representation of some generic object identified by ContentType and PK.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericObject {
    #[serde(rename = "object_type")]
    pub object_type: String,
    #[serde(rename = "object_id")]
    pub object_id: i32,
    #[serde(rename = "object", deserialize_with = "Option::deserialize")]
    pub object: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl GenericObject {
    /// Minimal representation of some generic object identified by ContentType and PK.
    pub fn new(object_type: String, object_id: i32, object: Option<::std::collections::HashMap<String, serde_json::Value>>) -> GenericObject {
        GenericObject {
            object_type,
            object_id,
            object,
        }
    }
}


