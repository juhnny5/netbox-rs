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
pub struct WritableCable {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "a_terminations", skip_serializing_if = "Option::is_none")]
    pub a_terminations: Option<Vec<crate::models::GenericObject>>,
    #[serde(rename = "b_terminations", skip_serializing_if = "Option::is_none")]
    pub b_terminations: Option<Vec<crate::models::GenericObject>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "length", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub length: Option<Option<f32>>,
    #[serde(rename = "length_unit", skip_serializing_if = "Option::is_none")]
    pub length_unit: Option<LengthUnit>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
}

impl WritableCable {
    pub fn new() -> WritableCable {
        WritableCable {
            id: None,
            url: None,
            display: None,
            r#type: None,
            a_terminations: None,
            b_terminations: None,
            status: None,
            tenant: None,
            label: None,
            color: None,
            length: None,
            length_unit: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "cat3")]
    Cat3,
    #[serde(rename = "cat5")]
    Cat5,
    #[serde(rename = "cat5e")]
    Cat5e,
    #[serde(rename = "cat6")]
    Cat6,
    #[serde(rename = "cat6a")]
    Cat6a,
    #[serde(rename = "cat7")]
    Cat7,
    #[serde(rename = "cat7a")]
    Cat7a,
    #[serde(rename = "cat8")]
    Cat8,
    #[serde(rename = "dac-active")]
    DacActive,
    #[serde(rename = "dac-passive")]
    DacPassive,
    #[serde(rename = "mrj21-trunk")]
    Mrj21Trunk,
    #[serde(rename = "coaxial")]
    Coaxial,
    #[serde(rename = "mmf")]
    Mmf,
    #[serde(rename = "mmf-om1")]
    MmfOm1,
    #[serde(rename = "mmf-om2")]
    MmfOm2,
    #[serde(rename = "mmf-om3")]
    MmfOm3,
    #[serde(rename = "mmf-om4")]
    MmfOm4,
    #[serde(rename = "mmf-om5")]
    MmfOm5,
    #[serde(rename = "smf")]
    Smf,
    #[serde(rename = "smf-os1")]
    SmfOs1,
    #[serde(rename = "smf-os2")]
    SmfOs2,
    #[serde(rename = "aoc")]
    Aoc,
    #[serde(rename = "power")]
    Power,
}

impl Default for Type {
    fn default() -> Type {
        Self::Cat3
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "decommissioning")]
    Decommissioning,
}

impl Default for Status {
    fn default() -> Status {
        Self::Connected
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LengthUnit {
    #[serde(rename = "km")]
    Km,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "cm")]
    Cm,
    #[serde(rename = "mi")]
    Mi,
    #[serde(rename = "ft")]
    Ft,
    #[serde(rename = "in")]
    In,
}

impl Default for LengthUnit {
    fn default() -> LengthUnit {
        Self::Km
    }
}

