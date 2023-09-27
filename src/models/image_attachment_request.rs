/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ImageAttachmentRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageAttachmentRequest {
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "object_id")]
    pub object_id: i64,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "image")]
    pub image: std::path::PathBuf,
    #[serde(rename = "image_height")]
    pub image_height: i32,
    #[serde(rename = "image_width")]
    pub image_width: i32,
}

impl ImageAttachmentRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(content_type: String, object_id: i64, image: std::path::PathBuf, image_height: i32, image_width: i32) -> ImageAttachmentRequest {
        ImageAttachmentRequest {
            content_type,
            object_id,
            name: None,
            image,
            image_height,
            image_width,
        }
    }
}


