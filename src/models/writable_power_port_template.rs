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
pub struct WritablePowerPortTemplate {
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
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Maximum power draw (watts)
    #[serde(rename = "maximum_draw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub maximum_draw: Option<Option<i32>>,
    /// Allocated power draw (watts)
    #[serde(rename = "allocated_draw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allocated_draw: Option<Option<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
}

impl WritablePowerPortTemplate {
    pub fn new(name: String) -> WritablePowerPortTemplate {
        WritablePowerPortTemplate {
            id: None,
            url: None,
            display: None,
            device_type: None,
            module_type: None,
            name,
            label: None,
            r#type: None,
            maximum_draw: None,
            allocated_draw: None,
            description: None,
            created: None,
            last_updated: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "iec-60320-c6")]
    Iec60320C6,
    #[serde(rename = "iec-60320-c8")]
    Iec60320C8,
    #[serde(rename = "iec-60320-c14")]
    Iec60320C14,
    #[serde(rename = "iec-60320-c16")]
    Iec60320C16,
    #[serde(rename = "iec-60320-c20")]
    Iec60320C20,
    #[serde(rename = "iec-60320-c22")]
    Iec60320C22,
    #[serde(rename = "iec-60309-p-n-e-4h")]
    Iec60309PNE4h,
    #[serde(rename = "iec-60309-p-n-e-6h")]
    Iec60309PNE6h,
    #[serde(rename = "iec-60309-p-n-e-9h")]
    Iec60309PNE9h,
    #[serde(rename = "iec-60309-2p-e-4h")]
    Iec603092pE4h,
    #[serde(rename = "iec-60309-2p-e-6h")]
    Iec603092pE6h,
    #[serde(rename = "iec-60309-2p-e-9h")]
    Iec603092pE9h,
    #[serde(rename = "iec-60309-3p-e-4h")]
    Iec603093pE4h,
    #[serde(rename = "iec-60309-3p-e-6h")]
    Iec603093pE6h,
    #[serde(rename = "iec-60309-3p-e-9h")]
    Iec603093pE9h,
    #[serde(rename = "iec-60309-3p-n-e-4h")]
    Iec603093pNE4h,
    #[serde(rename = "iec-60309-3p-n-e-6h")]
    Iec603093pNE6h,
    #[serde(rename = "iec-60309-3p-n-e-9h")]
    Iec603093pNE9h,
    #[serde(rename = "nema-1-15p")]
    Nema115p,
    #[serde(rename = "nema-5-15p")]
    Nema515p,
    #[serde(rename = "nema-5-20p")]
    Nema520p,
    #[serde(rename = "nema-5-30p")]
    Nema530p,
    #[serde(rename = "nema-5-50p")]
    Nema550p,
    #[serde(rename = "nema-6-15p")]
    Nema615p,
    #[serde(rename = "nema-6-20p")]
    Nema620p,
    #[serde(rename = "nema-6-30p")]
    Nema630p,
    #[serde(rename = "nema-6-50p")]
    Nema650p,
    #[serde(rename = "nema-10-30p")]
    Nema1030p,
    #[serde(rename = "nema-10-50p")]
    Nema1050p,
    #[serde(rename = "nema-14-20p")]
    Nema1420p,
    #[serde(rename = "nema-14-30p")]
    Nema1430p,
    #[serde(rename = "nema-14-50p")]
    Nema1450p,
    #[serde(rename = "nema-14-60p")]
    Nema1460p,
    #[serde(rename = "nema-15-15p")]
    Nema1515p,
    #[serde(rename = "nema-15-20p")]
    Nema1520p,
    #[serde(rename = "nema-15-30p")]
    Nema1530p,
    #[serde(rename = "nema-15-50p")]
    Nema1550p,
    #[serde(rename = "nema-15-60p")]
    Nema1560p,
    #[serde(rename = "nema-l1-15p")]
    NemaL115p,
    #[serde(rename = "nema-l5-15p")]
    NemaL515p,
    #[serde(rename = "nema-l5-20p")]
    NemaL520p,
    #[serde(rename = "nema-l5-30p")]
    NemaL530p,
    #[serde(rename = "nema-l5-50p")]
    NemaL550p,
    #[serde(rename = "nema-l6-15p")]
    NemaL615p,
    #[serde(rename = "nema-l6-20p")]
    NemaL620p,
    #[serde(rename = "nema-l6-30p")]
    NemaL630p,
    #[serde(rename = "nema-l6-50p")]
    NemaL650p,
    #[serde(rename = "nema-l10-30p")]
    NemaL1030p,
    #[serde(rename = "nema-l14-20p")]
    NemaL1420p,
    #[serde(rename = "nema-l14-30p")]
    NemaL1430p,
    #[serde(rename = "nema-l14-50p")]
    NemaL1450p,
    #[serde(rename = "nema-l14-60p")]
    NemaL1460p,
    #[serde(rename = "nema-l15-20p")]
    NemaL1520p,
    #[serde(rename = "nema-l15-30p")]
    NemaL1530p,
    #[serde(rename = "nema-l15-50p")]
    NemaL1550p,
    #[serde(rename = "nema-l15-60p")]
    NemaL1560p,
    #[serde(rename = "nema-l21-20p")]
    NemaL2120p,
    #[serde(rename = "nema-l21-30p")]
    NemaL2130p,
    #[serde(rename = "nema-l22-30p")]
    NemaL2230p,
    #[serde(rename = "cs6361c")]
    Cs6361c,
    #[serde(rename = "cs6365c")]
    Cs6365c,
    #[serde(rename = "cs8165c")]
    Cs8165c,
    #[serde(rename = "cs8265c")]
    Cs8265c,
    #[serde(rename = "cs8365c")]
    Cs8365c,
    #[serde(rename = "cs8465c")]
    Cs8465c,
    #[serde(rename = "ita-c")]
    ItaC,
    #[serde(rename = "ita-e")]
    ItaE,
    #[serde(rename = "ita-f")]
    ItaF,
    #[serde(rename = "ita-ef")]
    ItaEf,
    #[serde(rename = "ita-g")]
    ItaG,
    #[serde(rename = "ita-h")]
    ItaH,
    #[serde(rename = "ita-i")]
    ItaI,
    #[serde(rename = "ita-j")]
    ItaJ,
    #[serde(rename = "ita-k")]
    ItaK,
    #[serde(rename = "ita-l")]
    ItaL,
    #[serde(rename = "ita-m")]
    ItaM,
    #[serde(rename = "ita-n")]
    ItaN,
    #[serde(rename = "ita-o")]
    ItaO,
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
    #[serde(rename = "usb-3-b")]
    Usb3B,
    #[serde(rename = "usb-3-micro-b")]
    Usb3MicroB,
    #[serde(rename = "dc-terminal")]
    DcTerminal,
    #[serde(rename = "saf-d-grid")]
    SafDGrid,
    #[serde(rename = "neutrik-powercon-20")]
    NeutrikPowercon20,
    #[serde(rename = "neutrik-powercon-32")]
    NeutrikPowercon32,
    #[serde(rename = "neutrik-powercon-true1")]
    NeutrikPowerconTrue1,
    #[serde(rename = "neutrik-powercon-true1-top")]
    NeutrikPowerconTrue1Top,
    #[serde(rename = "ubiquiti-smartpower")]
    UbiquitiSmartpower,
    #[serde(rename = "hardwired")]
    Hardwired,
    #[serde(rename = "other")]
    Other,
}

impl Default for Type {
    fn default() -> Type {
        Self::Iec60320C6
    }
}

