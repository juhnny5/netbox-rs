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
pub struct DataSourceType {
    /// * `local` - Local * `git` - Git * `amazon-s3` - Amazon S3
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl DataSourceType {
    pub fn new() -> DataSourceType {
        DataSourceType {
            value: None,
            label: None,
        }
    }
}

/// * `local` - Local * `git` - Git * `amazon-s3` - Amazon S3
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "amazon-s3")]
    AmazonS3,
}

impl Default for Value {
    fn default() -> Value {
        Self::Local
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Local")]
    Local,
    #[serde(rename = "Git")]
    Git,
    #[serde(rename = "Amazon S3")]
    AmazonS3,
}

impl Default for Label {
    fn default() -> Label {
        Self::Local
    }
}

