/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WritablePowerOutletTemplateRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritablePowerOutletTemplateRequest {
    #[serde(rename = "device_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<Option<i32>>,
    #[serde(rename = "module_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module_type: Option<Option<i32>>,
    /// {module} is accepted as a substitution for the module bay position when attached to a module type.
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// * `iec-60320-c5` - C5 * `iec-60320-c7` - C7 * `iec-60320-c13` - C13 * `iec-60320-c15` - C15 * `iec-60320-c19` - C19 * `iec-60320-c21` - C21 * `iec-60309-p-n-e-4h` - P+N+E 4H * `iec-60309-p-n-e-6h` - P+N+E 6H * `iec-60309-p-n-e-9h` - P+N+E 9H * `iec-60309-2p-e-4h` - 2P+E 4H * `iec-60309-2p-e-6h` - 2P+E 6H * `iec-60309-2p-e-9h` - 2P+E 9H * `iec-60309-3p-e-4h` - 3P+E 4H * `iec-60309-3p-e-6h` - 3P+E 6H * `iec-60309-3p-e-9h` - 3P+E 9H * `iec-60309-3p-n-e-4h` - 3P+N+E 4H * `iec-60309-3p-n-e-6h` - 3P+N+E 6H * `iec-60309-3p-n-e-9h` - 3P+N+E 9H * `iec-60906-1` - IEC 60906-1 * `nbr-14136-10a` - 2P+T 10A (NBR 14136) * `nbr-14136-20a` - 2P+T 20A (NBR 14136) * `nema-1-15r` - NEMA 1-15R * `nema-5-15r` - NEMA 5-15R * `nema-5-20r` - NEMA 5-20R * `nema-5-30r` - NEMA 5-30R * `nema-5-50r` - NEMA 5-50R * `nema-6-15r` - NEMA 6-15R * `nema-6-20r` - NEMA 6-20R * `nema-6-30r` - NEMA 6-30R * `nema-6-50r` - NEMA 6-50R * `nema-10-30r` - NEMA 10-30R * `nema-10-50r` - NEMA 10-50R * `nema-14-20r` - NEMA 14-20R * `nema-14-30r` - NEMA 14-30R * `nema-14-50r` - NEMA 14-50R * `nema-14-60r` - NEMA 14-60R * `nema-15-15r` - NEMA 15-15R * `nema-15-20r` - NEMA 15-20R * `nema-15-30r` - NEMA 15-30R * `nema-15-50r` - NEMA 15-50R * `nema-15-60r` - NEMA 15-60R * `nema-l1-15r` - NEMA L1-15R * `nema-l5-15r` - NEMA L5-15R * `nema-l5-20r` - NEMA L5-20R * `nema-l5-30r` - NEMA L5-30R * `nema-l5-50r` - NEMA L5-50R * `nema-l6-15r` - NEMA L6-15R * `nema-l6-20r` - NEMA L6-20R * `nema-l6-30r` - NEMA L6-30R * `nema-l6-50r` - NEMA L6-50R * `nema-l10-30r` - NEMA L10-30R * `nema-l14-20r` - NEMA L14-20R * `nema-l14-30r` - NEMA L14-30R * `nema-l14-50r` - NEMA L14-50R * `nema-l14-60r` - NEMA L14-60R * `nema-l15-20r` - NEMA L15-20R * `nema-l15-30r` - NEMA L15-30R * `nema-l15-50r` - NEMA L15-50R * `nema-l15-60r` - NEMA L15-60R * `nema-l21-20r` - NEMA L21-20R * `nema-l21-30r` - NEMA L21-30R * `nema-l22-30r` - NEMA L22-30R * `CS6360C` - CS6360C * `CS6364C` - CS6364C * `CS8164C` - CS8164C * `CS8264C` - CS8264C * `CS8364C` - CS8364C * `CS8464C` - CS8464C * `ita-e` - ITA Type E (CEE 7/5) * `ita-f` - ITA Type F (CEE 7/3) * `ita-g` - ITA Type G (BS 1363) * `ita-h` - ITA Type H * `ita-i` - ITA Type I * `ita-j` - ITA Type J * `ita-k` - ITA Type K * `ita-l` - ITA Type L (CEI 23-50) * `ita-m` - ITA Type M (BS 546) * `ita-n` - ITA Type N * `ita-o` - ITA Type O * `ita-multistandard` - ITA Multistandard * `usb-a` - USB Type A * `usb-micro-b` - USB Micro B * `usb-c` - USB Type C * `dc-terminal` - DC Terminal * `hdot-cx` - HDOT Cx * `saf-d-grid` - Saf-D-Grid * `neutrik-powercon-20a` - Neutrik powerCON (20A) * `neutrik-powercon-32a` - Neutrik powerCON (32A) * `neutrik-powercon-true1` - Neutrik powerCON TRUE1 * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP * `ubiquiti-smartpower` - Ubiquiti SmartPower * `hardwired` - Hardwired * `other` - Other
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "power_port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub power_port: Option<Option<i32>>,
    /// Phase (for three-phase feeds)  * `A` - A * `B` - B * `C` - C
    #[serde(rename = "feed_leg", skip_serializing_if = "Option::is_none")]
    pub feed_leg: Option<FeedLeg>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl WritablePowerOutletTemplateRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(name: String) -> WritablePowerOutletTemplateRequest {
        WritablePowerOutletTemplateRequest {
            device_type: None,
            module_type: None,
            name,
            label: None,
            r#type: None,
            power_port: None,
            feed_leg: None,
            description: None,
        }
    }
}

/// * `iec-60320-c5` - C5 * `iec-60320-c7` - C7 * `iec-60320-c13` - C13 * `iec-60320-c15` - C15 * `iec-60320-c19` - C19 * `iec-60320-c21` - C21 * `iec-60309-p-n-e-4h` - P+N+E 4H * `iec-60309-p-n-e-6h` - P+N+E 6H * `iec-60309-p-n-e-9h` - P+N+E 9H * `iec-60309-2p-e-4h` - 2P+E 4H * `iec-60309-2p-e-6h` - 2P+E 6H * `iec-60309-2p-e-9h` - 2P+E 9H * `iec-60309-3p-e-4h` - 3P+E 4H * `iec-60309-3p-e-6h` - 3P+E 6H * `iec-60309-3p-e-9h` - 3P+E 9H * `iec-60309-3p-n-e-4h` - 3P+N+E 4H * `iec-60309-3p-n-e-6h` - 3P+N+E 6H * `iec-60309-3p-n-e-9h` - 3P+N+E 9H * `iec-60906-1` - IEC 60906-1 * `nbr-14136-10a` - 2P+T 10A (NBR 14136) * `nbr-14136-20a` - 2P+T 20A (NBR 14136) * `nema-1-15r` - NEMA 1-15R * `nema-5-15r` - NEMA 5-15R * `nema-5-20r` - NEMA 5-20R * `nema-5-30r` - NEMA 5-30R * `nema-5-50r` - NEMA 5-50R * `nema-6-15r` - NEMA 6-15R * `nema-6-20r` - NEMA 6-20R * `nema-6-30r` - NEMA 6-30R * `nema-6-50r` - NEMA 6-50R * `nema-10-30r` - NEMA 10-30R * `nema-10-50r` - NEMA 10-50R * `nema-14-20r` - NEMA 14-20R * `nema-14-30r` - NEMA 14-30R * `nema-14-50r` - NEMA 14-50R * `nema-14-60r` - NEMA 14-60R * `nema-15-15r` - NEMA 15-15R * `nema-15-20r` - NEMA 15-20R * `nema-15-30r` - NEMA 15-30R * `nema-15-50r` - NEMA 15-50R * `nema-15-60r` - NEMA 15-60R * `nema-l1-15r` - NEMA L1-15R * `nema-l5-15r` - NEMA L5-15R * `nema-l5-20r` - NEMA L5-20R * `nema-l5-30r` - NEMA L5-30R * `nema-l5-50r` - NEMA L5-50R * `nema-l6-15r` - NEMA L6-15R * `nema-l6-20r` - NEMA L6-20R * `nema-l6-30r` - NEMA L6-30R * `nema-l6-50r` - NEMA L6-50R * `nema-l10-30r` - NEMA L10-30R * `nema-l14-20r` - NEMA L14-20R * `nema-l14-30r` - NEMA L14-30R * `nema-l14-50r` - NEMA L14-50R * `nema-l14-60r` - NEMA L14-60R * `nema-l15-20r` - NEMA L15-20R * `nema-l15-30r` - NEMA L15-30R * `nema-l15-50r` - NEMA L15-50R * `nema-l15-60r` - NEMA L15-60R * `nema-l21-20r` - NEMA L21-20R * `nema-l21-30r` - NEMA L21-30R * `nema-l22-30r` - NEMA L22-30R * `CS6360C` - CS6360C * `CS6364C` - CS6364C * `CS8164C` - CS8164C * `CS8264C` - CS8264C * `CS8364C` - CS8364C * `CS8464C` - CS8464C * `ita-e` - ITA Type E (CEE 7/5) * `ita-f` - ITA Type F (CEE 7/3) * `ita-g` - ITA Type G (BS 1363) * `ita-h` - ITA Type H * `ita-i` - ITA Type I * `ita-j` - ITA Type J * `ita-k` - ITA Type K * `ita-l` - ITA Type L (CEI 23-50) * `ita-m` - ITA Type M (BS 546) * `ita-n` - ITA Type N * `ita-o` - ITA Type O * `ita-multistandard` - ITA Multistandard * `usb-a` - USB Type A * `usb-micro-b` - USB Micro B * `usb-c` - USB Type C * `dc-terminal` - DC Terminal * `hdot-cx` - HDOT Cx * `saf-d-grid` - Saf-D-Grid * `neutrik-powercon-20a` - Neutrik powerCON (20A) * `neutrik-powercon-32a` - Neutrik powerCON (32A) * `neutrik-powercon-true1` - Neutrik powerCON TRUE1 * `neutrik-powercon-true1-top` - Neutrik powerCON TRUE1 TOP * `ubiquiti-smartpower` - Ubiquiti SmartPower * `hardwired` - Hardwired * `other` - Other
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "iec-60320-c5")]
    Iec60320C5,
    #[serde(rename = "iec-60320-c7")]
    Iec60320C7,
    #[serde(rename = "iec-60320-c13")]
    Iec60320C13,
    #[serde(rename = "iec-60320-c15")]
    Iec60320C15,
    #[serde(rename = "iec-60320-c19")]
    Iec60320C19,
    #[serde(rename = "iec-60320-c21")]
    Iec60320C21,
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
    #[serde(rename = "iec-60906-1")]
    Iec609061,
    #[serde(rename = "nbr-14136-10a")]
    Nbr1413610a,
    #[serde(rename = "nbr-14136-20a")]
    Nbr1413620a,
    #[serde(rename = "nema-1-15r")]
    Nema115r,
    #[serde(rename = "nema-5-15r")]
    Nema515r,
    #[serde(rename = "nema-5-20r")]
    Nema520r,
    #[serde(rename = "nema-5-30r")]
    Nema530r,
    #[serde(rename = "nema-5-50r")]
    Nema550r,
    #[serde(rename = "nema-6-15r")]
    Nema615r,
    #[serde(rename = "nema-6-20r")]
    Nema620r,
    #[serde(rename = "nema-6-30r")]
    Nema630r,
    #[serde(rename = "nema-6-50r")]
    Nema650r,
    #[serde(rename = "nema-10-30r")]
    Nema1030r,
    #[serde(rename = "nema-10-50r")]
    Nema1050r,
    #[serde(rename = "nema-14-20r")]
    Nema1420r,
    #[serde(rename = "nema-14-30r")]
    Nema1430r,
    #[serde(rename = "nema-14-50r")]
    Nema1450r,
    #[serde(rename = "nema-14-60r")]
    Nema1460r,
    #[serde(rename = "nema-15-15r")]
    Nema1515r,
    #[serde(rename = "nema-15-20r")]
    Nema1520r,
    #[serde(rename = "nema-15-30r")]
    Nema1530r,
    #[serde(rename = "nema-15-50r")]
    Nema1550r,
    #[serde(rename = "nema-15-60r")]
    Nema1560r,
    #[serde(rename = "nema-l1-15r")]
    NemaL115r,
    #[serde(rename = "nema-l5-15r")]
    NemaL515r,
    #[serde(rename = "nema-l5-20r")]
    NemaL520r,
    #[serde(rename = "nema-l5-30r")]
    NemaL530r,
    #[serde(rename = "nema-l5-50r")]
    NemaL550r,
    #[serde(rename = "nema-l6-15r")]
    NemaL615r,
    #[serde(rename = "nema-l6-20r")]
    NemaL620r,
    #[serde(rename = "nema-l6-30r")]
    NemaL630r,
    #[serde(rename = "nema-l6-50r")]
    NemaL650r,
    #[serde(rename = "nema-l10-30r")]
    NemaL1030r,
    #[serde(rename = "nema-l14-20r")]
    NemaL1420r,
    #[serde(rename = "nema-l14-30r")]
    NemaL1430r,
    #[serde(rename = "nema-l14-50r")]
    NemaL1450r,
    #[serde(rename = "nema-l14-60r")]
    NemaL1460r,
    #[serde(rename = "nema-l15-20r")]
    NemaL1520r,
    #[serde(rename = "nema-l15-30r")]
    NemaL1530r,
    #[serde(rename = "nema-l15-50r")]
    NemaL1550r,
    #[serde(rename = "nema-l15-60r")]
    NemaL1560r,
    #[serde(rename = "nema-l21-20r")]
    NemaL2120r,
    #[serde(rename = "nema-l21-30r")]
    NemaL2130r,
    #[serde(rename = "nema-l22-30r")]
    NemaL2230r,
    #[serde(rename = "CS6360C")]
    Cs6360C,
    #[serde(rename = "CS6364C")]
    Cs6364C,
    #[serde(rename = "CS8164C")]
    Cs8164C,
    #[serde(rename = "CS8264C")]
    Cs8264C,
    #[serde(rename = "CS8364C")]
    Cs8364C,
    #[serde(rename = "CS8464C")]
    Cs8464C,
    #[serde(rename = "ita-e")]
    ItaE,
    #[serde(rename = "ita-f")]
    ItaF,
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
    #[serde(rename = "ita-multistandard")]
    ItaMultistandard,
    #[serde(rename = "usb-a")]
    UsbA,
    #[serde(rename = "usb-micro-b")]
    UsbMicroB,
    #[serde(rename = "usb-c")]
    UsbC,
    #[serde(rename = "dc-terminal")]
    DcTerminal,
    #[serde(rename = "hdot-cx")]
    HdotCx,
    #[serde(rename = "saf-d-grid")]
    SafDGrid,
    #[serde(rename = "neutrik-powercon-20a")]
    NeutrikPowercon20a,
    #[serde(rename = "neutrik-powercon-32a")]
    NeutrikPowercon32a,
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
    #[serde(rename = "")]
    Empty,
}

impl Default for Type {
    fn default() -> Type {
        Self::Iec60320C5
    }
}
/// Phase (for three-phase feeds)  * `A` - A * `B` - B * `C` - C
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeedLeg {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "")]
    Empty,
}

impl Default for FeedLeg {
    fn default() -> FeedLeg {
        Self::A
    }
}

