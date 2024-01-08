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
pub struct Type {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Type {
    pub fn new(label: Label, value: Value) -> Type {
        Type {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "DE-9")]
    De9,
    #[serde(rename = "DB-25")]
    Db25,
    #[serde(rename = "RJ-11")]
    Rj11,
    #[serde(rename = "RJ-12")]
    Rj12,
    #[serde(rename = "RJ-45")]
    Rj45,
    #[serde(rename = "Mini-DIN 8")]
    MiniDin8,
    #[serde(rename = "USB Type A")]
    UsbTypeA,
    #[serde(rename = "USB Type B")]
    UsbTypeB,
    #[serde(rename = "USB Type C")]
    UsbTypeC,
    #[serde(rename = "USB Mini A")]
    UsbMiniA,
    #[serde(rename = "USB Mini B")]
    UsbMiniB,
    #[serde(rename = "USB Micro A")]
    UsbMicroA,
    #[serde(rename = "USB Micro B")]
    UsbMicroB,
    #[serde(rename = "USB Micro AB")]
    UsbMicroAb,
    #[serde(rename = "Other")]
    Other,
}

impl Default for Label {
    fn default() -> Label {
        Self::De9
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "de-9")]
    De9,
    #[serde(rename = "db-25")]
    Db25,
    #[serde(rename = "rj-11")]
    Rj11,
    #[serde(rename = "rj-12")]
    Rj12,
    #[serde(rename = "rj-45")]
    Rj45,
    #[serde(rename = "mini-din-8")]
    MiniDin8,
    #[serde(rename = "usb-a")]
    UsbA,
    #[serde(rename = "usb-b")]
    UsbB,
    #[serde(rename = "usb-c")]
    UsbC,
    #[serde(rename = "usb-mini-a")]
    UsbMiniA,
    #[serde(rename = "usb-mini-b")]
    UsbMiniB,
    #[serde(rename = "usb-micro-a")]
    UsbMicroA,
    #[serde(rename = "usb-micro-b")]
    UsbMicroB,
    #[serde(rename = "usb-micro-ab")]
    UsbMicroAb,
    #[serde(rename = "other")]
    Other,
}

impl Default for Value {
    fn default() -> Value {
        Self::De9
    }
}

