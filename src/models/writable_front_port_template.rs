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
pub struct WritableFrontPortTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<Option<i32>>,
    #[serde(rename = "module_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module_type: Option<Option<i32>>,
    ///  {module} is accepted as a substitution for the module bay position when attached to a module type. 
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "rear_port")]
    pub rear_port: i32,
    #[serde(rename = "rear_port_position", skip_serializing_if = "Option::is_none")]
    pub rear_port_position: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
}

impl WritableFrontPortTemplate {
    pub fn new(name: String, r#type: Type, rear_port: i32) -> WritableFrontPortTemplate {
        WritableFrontPortTemplate {
            id: None,
            url: None,
            display: None,
            device_type: None,
            module_type: None,
            name,
            label: None,
            r#type,
            color: None,
            rear_port,
            rear_port_position: None,
            description: None,
            created: None,
            last_updated: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "8p8c")]
    Variant8p8c,
    #[serde(rename = "8p6c")]
    Variant8p6c,
    #[serde(rename = "8p4c")]
    Variant8p4c,
    #[serde(rename = "8p2c")]
    Variant8p2c,
    #[serde(rename = "6p6c")]
    Variant6p6c,
    #[serde(rename = "6p4c")]
    Variant6p4c,
    #[serde(rename = "6p2c")]
    Variant6p2c,
    #[serde(rename = "4p4c")]
    Variant4p4c,
    #[serde(rename = "4p2c")]
    Variant4p2c,
    #[serde(rename = "gg45")]
    Gg45,
    #[serde(rename = "tera-4p")]
    Tera4p,
    #[serde(rename = "tera-2p")]
    Tera2p,
    #[serde(rename = "tera-1p")]
    Tera1p,
    #[serde(rename = "110-punch")]
    Variant110Punch,
    #[serde(rename = "bnc")]
    Bnc,
    #[serde(rename = "f")]
    F,
    #[serde(rename = "n")]
    N,
    #[serde(rename = "mrj21")]
    Mrj21,
    #[serde(rename = "fc")]
    Fc,
    #[serde(rename = "lc")]
    Lc,
    #[serde(rename = "lc-pc")]
    LcPc,
    #[serde(rename = "lc-upc")]
    LcUpc,
    #[serde(rename = "lc-apc")]
    LcApc,
    #[serde(rename = "lsh")]
    Lsh,
    #[serde(rename = "lsh-pc")]
    LshPc,
    #[serde(rename = "lsh-upc")]
    LshUpc,
    #[serde(rename = "lsh-apc")]
    LshApc,
    #[serde(rename = "mpo")]
    Mpo,
    #[serde(rename = "mtrj")]
    Mtrj,
    #[serde(rename = "sc")]
    Sc,
    #[serde(rename = "sc-pc")]
    ScPc,
    #[serde(rename = "sc-upc")]
    ScUpc,
    #[serde(rename = "sc-apc")]
    ScApc,
    #[serde(rename = "st")]
    St,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "sn")]
    Sn,
    #[serde(rename = "sma-905")]
    Sma905,
    #[serde(rename = "sma-906")]
    Sma906,
    #[serde(rename = "urm-p2")]
    UrmP2,
    #[serde(rename = "urm-p4")]
    UrmP4,
    #[serde(rename = "urm-p8")]
    UrmP8,
    #[serde(rename = "splice")]
    Splice,
    #[serde(rename = "other")]
    Other,
}

impl Default for Type {
    fn default() -> Type {
        Self::Variant8p8c
    }
}

