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
pub struct Type6 {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Type6 {
    pub fn new(label: Label, value: Value) -> Type6 {
        Type6 {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "2-post frame")]
    Variant2PostFrame,
    #[serde(rename = "4-post frame")]
    Variant4PostFrame,
    #[serde(rename = "4-post cabinet")]
    Variant4PostCabinet,
    #[serde(rename = "Wall-mounted frame")]
    WallMountedFrame,
    #[serde(rename = "Wall-mounted frame (vertical)")]
    WallMountedFrameLeftParenthesisVerticalRightParenthesis,
    #[serde(rename = "Wall-mounted cabinet")]
    WallMountedCabinet,
    #[serde(rename = "Wall-mounted cabinet (vertical)")]
    WallMountedCabinetLeftParenthesisVerticalRightParenthesis,
}

impl Default for Label {
    fn default() -> Label {
        Self::Variant2PostFrame
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
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

impl Default for Value {
    fn default() -> Value {
        Self::Variant2PostFrame
    }
}

