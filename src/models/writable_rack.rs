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
pub struct WritableRack {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "facility_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub facility_id: Option<Option<String>>,
    #[serde(rename = "site")]
    pub site: i32,
    #[serde(rename = "location", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<i32>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Functional role
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<i32>>,
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// A unique tag used to identify this rack
    #[serde(rename = "asset_tag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<Option<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Rail-to-rail width
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// Height in rack units
    #[serde(rename = "u_height", skip_serializing_if = "Option::is_none")]
    pub u_height: Option<i32>,
    #[serde(rename = "weight", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub weight: Option<Option<f32>>,
    /// Maximum load capacity for the rack
    #[serde(rename = "max_weight", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_weight: Option<Option<i32>>,
    #[serde(rename = "weight_unit", skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<WeightUnit>,
    /// Units are numbered top-to-bottom
    #[serde(rename = "desc_units", skip_serializing_if = "Option::is_none")]
    pub desc_units: Option<bool>,
    /// Outer dimension of rack (width)
    #[serde(rename = "outer_width", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub outer_width: Option<Option<i32>>,
    /// Outer dimension of rack (depth)
    #[serde(rename = "outer_depth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub outer_depth: Option<Option<i32>>,
    #[serde(rename = "outer_unit", skip_serializing_if = "Option::is_none")]
    pub outer_unit: Option<OuterUnit>,
    /// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
    #[serde(rename = "mounting_depth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mounting_depth: Option<Option<i32>>,
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
    #[serde(rename = "device_count", skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
    #[serde(rename = "powerfeed_count", skip_serializing_if = "Option::is_none")]
    pub powerfeed_count: Option<i32>,
}

impl WritableRack {
    pub fn new(name: String, site: i32) -> WritableRack {
        WritableRack {
            id: None,
            url: None,
            display: None,
            name,
            facility_id: None,
            site,
            location: None,
            tenant: None,
            status: None,
            role: None,
            serial: None,
            asset_tag: None,
            r#type: None,
            width: None,
            u_height: None,
            weight: None,
            max_weight: None,
            weight_unit: None,
            desc_units: None,
            outer_width: None,
            outer_depth: None,
            outer_unit: None,
            mounting_depth: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            device_count: None,
            powerfeed_count: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deprecated")]
    Deprecated,
}

impl Default for Status {
    fn default() -> Status {
        Self::Reserved
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "2-post-frame")]
    Variant2PostFrame,
    #[serde(rename = "4-post-frame")]
    Variant4PostFrame,
    #[serde(rename = "4-post-cabinet")]
    Variant4PostCabinet,
    #[serde(rename = "wall-frame")]
    WallFrame,
    #[serde(rename = "wall-frame-vertical")]
    WallFrameVertical,
    #[serde(rename = "wall-cabinet")]
    WallCabinet,
    #[serde(rename = "wall-cabinet-vertical")]
    WallCabinetVertical,
}

impl Default for Type {
    fn default() -> Type {
        Self::Variant2PostFrame
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WeightUnit {
    #[serde(rename = "kg")]
    Kg,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "lb")]
    Lb,
    #[serde(rename = "oz")]
    Oz,
}

impl Default for WeightUnit {
    fn default() -> WeightUnit {
        Self::Kg
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OuterUnit {
    #[serde(rename = "mm")]
    Mm,
    #[serde(rename = "in")]
    In,
}

impl Default for OuterUnit {
    fn default() -> OuterUnit {
        Self::Mm
    }
}

