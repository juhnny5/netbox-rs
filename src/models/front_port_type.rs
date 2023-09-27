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
pub struct FrontPortType {
    /// * `8p8c` - 8P8C * `8p6c` - 8P6C * `8p4c` - 8P4C * `8p2c` - 8P2C * `6p6c` - 6P6C * `6p4c` - 6P4C * `6p2c` - 6P2C * `4p4c` - 4P4C * `4p2c` - 4P2C * `gg45` - GG45 * `tera-4p` - TERA 4P * `tera-2p` - TERA 2P * `tera-1p` - TERA 1P * `110-punch` - 110 Punch * `bnc` - BNC * `f` - F Connector * `n` - N Connector * `mrj21` - MRJ21 * `fc` - FC * `lc` - LC * `lc-pc` - LC/PC * `lc-upc` - LC/UPC * `lc-apc` - LC/APC * `lsh` - LSH * `lsh-pc` - LSH/PC * `lsh-upc` - LSH/UPC * `lsh-apc` - LSH/APC * `lx5` - LX.5 * `lx5-pc` - LX.5/PC * `lx5-upc` - LX.5/UPC * `lx5-apc` - LX.5/APC * `mpo` - MPO * `mtrj` - MTRJ * `sc` - SC * `sc-pc` - SC/PC * `sc-upc` - SC/UPC * `sc-apc` - SC/APC * `st` - ST * `cs` - CS * `sn` - SN * `sma-905` - SMA 905 * `sma-906` - SMA 906 * `urm-p2` - URM-P2 * `urm-p4` - URM-P4 * `urm-p8` - URM-P8 * `splice` - Splice * `other` - Other
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
}

impl FrontPortType {
    pub fn new() -> FrontPortType {
        FrontPortType {
            value: None,
            label: None,
        }
    }
}

/// * `8p8c` - 8P8C * `8p6c` - 8P6C * `8p4c` - 8P4C * `8p2c` - 8P2C * `6p6c` - 6P6C * `6p4c` - 6P4C * `6p2c` - 6P2C * `4p4c` - 4P4C * `4p2c` - 4P2C * `gg45` - GG45 * `tera-4p` - TERA 4P * `tera-2p` - TERA 2P * `tera-1p` - TERA 1P * `110-punch` - 110 Punch * `bnc` - BNC * `f` - F Connector * `n` - N Connector * `mrj21` - MRJ21 * `fc` - FC * `lc` - LC * `lc-pc` - LC/PC * `lc-upc` - LC/UPC * `lc-apc` - LC/APC * `lsh` - LSH * `lsh-pc` - LSH/PC * `lsh-upc` - LSH/UPC * `lsh-apc` - LSH/APC * `lx5` - LX.5 * `lx5-pc` - LX.5/PC * `lx5-upc` - LX.5/UPC * `lx5-apc` - LX.5/APC * `mpo` - MPO * `mtrj` - MTRJ * `sc` - SC * `sc-pc` - SC/PC * `sc-upc` - SC/UPC * `sc-apc` - SC/APC * `st` - ST * `cs` - CS * `sn` - SN * `sma-905` - SMA 905 * `sma-906` - SMA 906 * `urm-p2` - URM-P2 * `urm-p4` - URM-P4 * `urm-p8` - URM-P8 * `splice` - Splice * `other` - Other
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
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
    #[serde(rename = "lx5")]
    Lx5,
    #[serde(rename = "lx5-pc")]
    Lx5Pc,
    #[serde(rename = "lx5-upc")]
    Lx5Upc,
    #[serde(rename = "lx5-apc")]
    Lx5Apc,
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

impl Default for Value {
    fn default() -> Value {
        Self::Variant8p8c
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "8P8C")]
    Variant8P8C,
    #[serde(rename = "8P6C")]
    Variant8P6C,
    #[serde(rename = "8P4C")]
    Variant8P4C,
    #[serde(rename = "8P2C")]
    Variant8P2C,
    #[serde(rename = "6P6C")]
    Variant6P6C,
    #[serde(rename = "6P4C")]
    Variant6P4C,
    #[serde(rename = "6P2C")]
    Variant6P2C,
    #[serde(rename = "4P4C")]
    Variant4P4C,
    #[serde(rename = "4P2C")]
    Variant4P2C,
    #[serde(rename = "GG45")]
    Gg45,
    #[serde(rename = "TERA 4P")]
    Tera4P,
    #[serde(rename = "TERA 2P")]
    Tera2P,
    #[serde(rename = "TERA 1P")]
    Tera1P,
    #[serde(rename = "110 Punch")]
    Variant110Punch,
    #[serde(rename = "BNC")]
    Bnc,
    #[serde(rename = "F Connector")]
    FConnector,
    #[serde(rename = "N Connector")]
    NConnector,
    #[serde(rename = "MRJ21")]
    Mrj21,
    #[serde(rename = "FC")]
    Fc,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LC/PC")]
    LcSlashPc,
    #[serde(rename = "LC/UPC")]
    LcSlashUpc,
    #[serde(rename = "LC/APC")]
    LcSlashApc,
    #[serde(rename = "LSH")]
    Lsh,
    #[serde(rename = "LSH/PC")]
    LshSlashPc,
    #[serde(rename = "LSH/UPC")]
    LshSlashUpc,
    #[serde(rename = "LSH/APC")]
    LshSlashApc,
    #[serde(rename = "LX.5")]
    LxPeriod5,
    #[serde(rename = "LX.5/PC")]
    LxPeriod5SlashPc,
    #[serde(rename = "LX.5/UPC")]
    LxPeriod5SlashUpc,
    #[serde(rename = "LX.5/APC")]
    LxPeriod5SlashApc,
    #[serde(rename = "MPO")]
    Mpo,
    #[serde(rename = "MTRJ")]
    Mtrj,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SC/PC")]
    ScSlashPc,
    #[serde(rename = "SC/UPC")]
    ScSlashUpc,
    #[serde(rename = "SC/APC")]
    ScSlashApc,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "CS")]
    Cs,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SMA 905")]
    Sma905,
    #[serde(rename = "SMA 906")]
    Sma906,
    #[serde(rename = "URM-P2")]
    UrmP2,
    #[serde(rename = "URM-P4")]
    UrmP4,
    #[serde(rename = "URM-P8")]
    UrmP8,
    #[serde(rename = "Splice")]
    Splice,
    #[serde(rename = "Other")]
    Other,
}

impl Default for Label {
    fn default() -> Label {
        Self::Variant8P8C
    }
}

