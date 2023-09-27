/*
 * NetBox REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 3.6.0 (3.6)
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InterfaceTemplateRequest : Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterfaceTemplateRequest {
    #[serde(rename = "device_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<Option<Box<crate::models::NestedDeviceTypeRequest>>>,
    #[serde(rename = "module_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module_type: Option<Option<Box<crate::models::NestedModuleTypeRequest>>>,
    /// {module} is accepted as a substitution for the module bay position when attached to a module type.
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// * `virtual` - Virtual * `bridge` - Bridge * `lag` - Link Aggregation Group (LAG) * `100base-fx` - 100BASE-FX (10/100ME FIBER) * `100base-lfx` - 100BASE-LFX (10/100ME FIBER) * `100base-tx` - 100BASE-TX (10/100ME) * `100base-t1` - 100BASE-T1 (10/100ME Single Pair) * `1000base-t` - 1000BASE-T (1GE) * `2.5gbase-t` - 2.5GBASE-T (2.5GE) * `5gbase-t` - 5GBASE-T (5GE) * `10gbase-t` - 10GBASE-T (10GE) * `10gbase-cx4` - 10GBASE-CX4 (10GE) * `1000base-x-gbic` - GBIC (1GE) * `1000base-x-sfp` - SFP (1GE) * `10gbase-x-sfpp` - SFP+ (10GE) * `10gbase-x-xfp` - XFP (10GE) * `10gbase-x-xenpak` - XENPAK (10GE) * `10gbase-x-x2` - X2 (10GE) * `25gbase-x-sfp28` - SFP28 (25GE) * `50gbase-x-sfp56` - SFP56 (50GE) * `40gbase-x-qsfpp` - QSFP+ (40GE) * `50gbase-x-sfp28` - QSFP28 (50GE) * `100gbase-x-cfp` - CFP (100GE) * `100gbase-x-cfp2` - CFP2 (100GE) * `200gbase-x-cfp2` - CFP2 (200GE) * `400gbase-x-cfp2` - CFP2 (400GE) * `100gbase-x-cfp4` - CFP4 (100GE) * `100gbase-x-cxp` - CXP (100GE) * `100gbase-x-cpak` - Cisco CPAK (100GE) * `100gbase-x-dsfp` - DSFP (100GE) * `100gbase-x-sfpdd` - SFP-DD (100GE) * `100gbase-x-qsfp28` - QSFP28 (100GE) * `100gbase-x-qsfpdd` - QSFP-DD (100GE) * `200gbase-x-qsfp56` - QSFP56 (200GE) * `200gbase-x-qsfpdd` - QSFP-DD (200GE) * `400gbase-x-qsfpdd` - QSFP-DD (400GE) * `400gbase-x-osfp` - OSFP (400GE) * `400gbase-x-cdfp` - CDFP (400GE) * `400gbase-x-cfp8` - CPF8 (400GE) * `800gbase-x-qsfpdd` - QSFP-DD (800GE) * `800gbase-x-osfp` - OSFP (800GE) * `1000base-kx` - 1000BASE-KX (1GE) * `10gbase-kr` - 10GBASE-KR (10GE) * `10gbase-kx4` - 10GBASE-KX4 (10GE) * `25gbase-kr` - 25GBASE-KR (25GE) * `40gbase-kr4` - 40GBASE-KR4 (40GE) * `50gbase-kr` - 50GBASE-KR (50GE) * `100gbase-kp4` - 100GBASE-KP4 (100GE) * `100gbase-kr2` - 100GBASE-KR2 (100GE) * `100gbase-kr4` - 100GBASE-KR4 (100GE) * `ieee802.11a` - IEEE 802.11a * `ieee802.11g` - IEEE 802.11b/g * `ieee802.11n` - IEEE 802.11n * `ieee802.11ac` - IEEE 802.11ac * `ieee802.11ad` - IEEE 802.11ad * `ieee802.11ax` - IEEE 802.11ax * `ieee802.11ay` - IEEE 802.11ay * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth) * `other-wireless` - Other (Wireless) * `gsm` - GSM * `cdma` - CDMA * `lte` - LTE * `sonet-oc3` - OC-3/STM-1 * `sonet-oc12` - OC-12/STM-4 * `sonet-oc48` - OC-48/STM-16 * `sonet-oc192` - OC-192/STM-64 * `sonet-oc768` - OC-768/STM-256 * `sonet-oc1920` - OC-1920/STM-640 * `sonet-oc3840` - OC-3840/STM-1234 * `1gfc-sfp` - SFP (1GFC) * `2gfc-sfp` - SFP (2GFC) * `4gfc-sfp` - SFP (4GFC) * `8gfc-sfpp` - SFP+ (8GFC) * `16gfc-sfpp` - SFP+ (16GFC) * `32gfc-sfp28` - SFP28 (32GFC) * `64gfc-qsfpp` - QSFP+ (64GFC) * `128gfc-qsfp28` - QSFP28 (128GFC) * `infiniband-sdr` - SDR (2 Gbps) * `infiniband-ddr` - DDR (4 Gbps) * `infiniband-qdr` - QDR (8 Gbps) * `infiniband-fdr10` - FDR10 (10 Gbps) * `infiniband-fdr` - FDR (13.5 Gbps) * `infiniband-edr` - EDR (25 Gbps) * `infiniband-hdr` - HDR (50 Gbps) * `infiniband-ndr` - NDR (100 Gbps) * `infiniband-xdr` - XDR (250 Gbps) * `t1` - T1 (1.544 Mbps) * `e1` - E1 (2.048 Mbps) * `t3` - T3 (45 Mbps) * `e3` - E3 (34 Mbps) * `xdsl` - xDSL * `docsis` - DOCSIS * `gpon` - GPON (2.5 Gbps / 1.25 Gps) * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps) * `xgs-pon` - XGS-PON (10 Gbps) * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps) * `epon` - EPON (1 Gbps) * `10g-epon` - 10G-EPON (10 Gbps) * `cisco-stackwise` - Cisco StackWise * `cisco-stackwise-plus` - Cisco StackWise Plus * `cisco-flexstack` - Cisco FlexStack * `cisco-flexstack-plus` - Cisco FlexStack Plus * `cisco-stackwise-80` - Cisco StackWise-80 * `cisco-stackwise-160` - Cisco StackWise-160 * `cisco-stackwise-320` - Cisco StackWise-320 * `cisco-stackwise-480` - Cisco StackWise-480 * `cisco-stackwise-1t` - Cisco StackWise-1T * `juniper-vcp` - Juniper VCP * `extreme-summitstack` - Extreme SummitStack * `extreme-summitstack-128` - Extreme SummitStack-128 * `extreme-summitstack-256` - Extreme SummitStack-256 * `extreme-summitstack-512` - Extreme SummitStack-512 * `other` - Other
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "mgmt_only", skip_serializing_if = "Option::is_none")]
    pub mgmt_only: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "bridge", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bridge: Option<Option<Box<crate::models::NestedInterfaceTemplateRequest>>>,
    /// * `pd` - PD * `pse` - PSE
    #[serde(rename = "poe_mode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub poe_mode: Option<Option<PoeMode>>,
    /// * `type1-ieee802.3af` - 802.3af (Type 1) * `type2-ieee802.3at` - 802.3at (Type 2) * `type3-ieee802.3bt` - 802.3bt (Type 3) * `type4-ieee802.3bt` - 802.3bt (Type 4) * `passive-24v-2pair` - Passive 24V (2-pair) * `passive-24v-4pair` - Passive 24V (4-pair) * `passive-48v-2pair` - Passive 48V (2-pair) * `passive-48v-4pair` - Passive 48V (4-pair)
    #[serde(rename = "poe_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub poe_type: Option<Option<PoeType>>,
    /// * `ap` - Access point * `station` - Station
    #[serde(rename = "rf_role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rf_role: Option<Option<RfRole>>,
}

impl InterfaceTemplateRequest {
    /// Extends the built-in ModelSerializer to enforce calling full_clean() on a copy of the associated instance during validation. (DRF does not do this by default; see https://github.com/encode/django-rest-framework/issues/3144)
    pub fn new(name: String, r#type: Type) -> InterfaceTemplateRequest {
        InterfaceTemplateRequest {
            device_type: None,
            module_type: None,
            name,
            label: None,
            r#type,
            enabled: None,
            mgmt_only: None,
            description: None,
            bridge: None,
            poe_mode: None,
            poe_type: None,
            rf_role: None,
        }
    }
}

/// * `virtual` - Virtual * `bridge` - Bridge * `lag` - Link Aggregation Group (LAG) * `100base-fx` - 100BASE-FX (10/100ME FIBER) * `100base-lfx` - 100BASE-LFX (10/100ME FIBER) * `100base-tx` - 100BASE-TX (10/100ME) * `100base-t1` - 100BASE-T1 (10/100ME Single Pair) * `1000base-t` - 1000BASE-T (1GE) * `2.5gbase-t` - 2.5GBASE-T (2.5GE) * `5gbase-t` - 5GBASE-T (5GE) * `10gbase-t` - 10GBASE-T (10GE) * `10gbase-cx4` - 10GBASE-CX4 (10GE) * `1000base-x-gbic` - GBIC (1GE) * `1000base-x-sfp` - SFP (1GE) * `10gbase-x-sfpp` - SFP+ (10GE) * `10gbase-x-xfp` - XFP (10GE) * `10gbase-x-xenpak` - XENPAK (10GE) * `10gbase-x-x2` - X2 (10GE) * `25gbase-x-sfp28` - SFP28 (25GE) * `50gbase-x-sfp56` - SFP56 (50GE) * `40gbase-x-qsfpp` - QSFP+ (40GE) * `50gbase-x-sfp28` - QSFP28 (50GE) * `100gbase-x-cfp` - CFP (100GE) * `100gbase-x-cfp2` - CFP2 (100GE) * `200gbase-x-cfp2` - CFP2 (200GE) * `400gbase-x-cfp2` - CFP2 (400GE) * `100gbase-x-cfp4` - CFP4 (100GE) * `100gbase-x-cxp` - CXP (100GE) * `100gbase-x-cpak` - Cisco CPAK (100GE) * `100gbase-x-dsfp` - DSFP (100GE) * `100gbase-x-sfpdd` - SFP-DD (100GE) * `100gbase-x-qsfp28` - QSFP28 (100GE) * `100gbase-x-qsfpdd` - QSFP-DD (100GE) * `200gbase-x-qsfp56` - QSFP56 (200GE) * `200gbase-x-qsfpdd` - QSFP-DD (200GE) * `400gbase-x-qsfpdd` - QSFP-DD (400GE) * `400gbase-x-osfp` - OSFP (400GE) * `400gbase-x-cdfp` - CDFP (400GE) * `400gbase-x-cfp8` - CPF8 (400GE) * `800gbase-x-qsfpdd` - QSFP-DD (800GE) * `800gbase-x-osfp` - OSFP (800GE) * `1000base-kx` - 1000BASE-KX (1GE) * `10gbase-kr` - 10GBASE-KR (10GE) * `10gbase-kx4` - 10GBASE-KX4 (10GE) * `25gbase-kr` - 25GBASE-KR (25GE) * `40gbase-kr4` - 40GBASE-KR4 (40GE) * `50gbase-kr` - 50GBASE-KR (50GE) * `100gbase-kp4` - 100GBASE-KP4 (100GE) * `100gbase-kr2` - 100GBASE-KR2 (100GE) * `100gbase-kr4` - 100GBASE-KR4 (100GE) * `ieee802.11a` - IEEE 802.11a * `ieee802.11g` - IEEE 802.11b/g * `ieee802.11n` - IEEE 802.11n * `ieee802.11ac` - IEEE 802.11ac * `ieee802.11ad` - IEEE 802.11ad * `ieee802.11ax` - IEEE 802.11ax * `ieee802.11ay` - IEEE 802.11ay * `ieee802.15.1` - IEEE 802.15.1 (Bluetooth) * `other-wireless` - Other (Wireless) * `gsm` - GSM * `cdma` - CDMA * `lte` - LTE * `sonet-oc3` - OC-3/STM-1 * `sonet-oc12` - OC-12/STM-4 * `sonet-oc48` - OC-48/STM-16 * `sonet-oc192` - OC-192/STM-64 * `sonet-oc768` - OC-768/STM-256 * `sonet-oc1920` - OC-1920/STM-640 * `sonet-oc3840` - OC-3840/STM-1234 * `1gfc-sfp` - SFP (1GFC) * `2gfc-sfp` - SFP (2GFC) * `4gfc-sfp` - SFP (4GFC) * `8gfc-sfpp` - SFP+ (8GFC) * `16gfc-sfpp` - SFP+ (16GFC) * `32gfc-sfp28` - SFP28 (32GFC) * `64gfc-qsfpp` - QSFP+ (64GFC) * `128gfc-qsfp28` - QSFP28 (128GFC) * `infiniband-sdr` - SDR (2 Gbps) * `infiniband-ddr` - DDR (4 Gbps) * `infiniband-qdr` - QDR (8 Gbps) * `infiniband-fdr10` - FDR10 (10 Gbps) * `infiniband-fdr` - FDR (13.5 Gbps) * `infiniband-edr` - EDR (25 Gbps) * `infiniband-hdr` - HDR (50 Gbps) * `infiniband-ndr` - NDR (100 Gbps) * `infiniband-xdr` - XDR (250 Gbps) * `t1` - T1 (1.544 Mbps) * `e1` - E1 (2.048 Mbps) * `t3` - T3 (45 Mbps) * `e3` - E3 (34 Mbps) * `xdsl` - xDSL * `docsis` - DOCSIS * `gpon` - GPON (2.5 Gbps / 1.25 Gps) * `xg-pon` - XG-PON (10 Gbps / 2.5 Gbps) * `xgs-pon` - XGS-PON (10 Gbps) * `ng-pon2` - NG-PON2 (TWDM-PON) (4x10 Gbps) * `epon` - EPON (1 Gbps) * `10g-epon` - 10G-EPON (10 Gbps) * `cisco-stackwise` - Cisco StackWise * `cisco-stackwise-plus` - Cisco StackWise Plus * `cisco-flexstack` - Cisco FlexStack * `cisco-flexstack-plus` - Cisco FlexStack Plus * `cisco-stackwise-80` - Cisco StackWise-80 * `cisco-stackwise-160` - Cisco StackWise-160 * `cisco-stackwise-320` - Cisco StackWise-320 * `cisco-stackwise-480` - Cisco StackWise-480 * `cisco-stackwise-1t` - Cisco StackWise-1T * `juniper-vcp` - Juniper VCP * `extreme-summitstack` - Extreme SummitStack * `extreme-summitstack-128` - Extreme SummitStack-128 * `extreme-summitstack-256` - Extreme SummitStack-256 * `extreme-summitstack-512` - Extreme SummitStack-512 * `other` - Other
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "virtual")]
    Virtual,
    #[serde(rename = "bridge")]
    Bridge,
    #[serde(rename = "lag")]
    Lag,
    #[serde(rename = "100base-fx")]
    Variant100baseFx,
    #[serde(rename = "100base-lfx")]
    Variant100baseLfx,
    #[serde(rename = "100base-tx")]
    Variant100baseTx,
    #[serde(rename = "100base-t1")]
    Variant100baseT1,
    #[serde(rename = "1000base-t")]
    Variant1000baseT,
    #[serde(rename = "2.5gbase-t")]
    Variant2Period5gbaseT,
    #[serde(rename = "5gbase-t")]
    Variant5gbaseT,
    #[serde(rename = "10gbase-t")]
    Variant10gbaseT,
    #[serde(rename = "10gbase-cx4")]
    Variant10gbaseCx4,
    #[serde(rename = "1000base-x-gbic")]
    Variant1000baseXGbic,
    #[serde(rename = "1000base-x-sfp")]
    Variant1000baseXSfp,
    #[serde(rename = "10gbase-x-sfpp")]
    Variant10gbaseXSfpp,
    #[serde(rename = "10gbase-x-xfp")]
    Variant10gbaseXXfp,
    #[serde(rename = "10gbase-x-xenpak")]
    Variant10gbaseXXenpak,
    #[serde(rename = "10gbase-x-x2")]
    Variant10gbaseXx2,
    #[serde(rename = "25gbase-x-sfp28")]
    Variant25gbaseXSfp28,
    #[serde(rename = "50gbase-x-sfp56")]
    Variant50gbaseXSfp56,
    #[serde(rename = "40gbase-x-qsfpp")]
    Variant40gbaseXQsfpp,
    #[serde(rename = "50gbase-x-sfp28")]
    Variant50gbaseXSfp28,
    #[serde(rename = "100gbase-x-cfp")]
    Variant100gbaseXCfp,
    #[serde(rename = "100gbase-x-cfp2")]
    Variant100gbaseXCfp2,
    #[serde(rename = "200gbase-x-cfp2")]
    Variant200gbaseXCfp2,
    #[serde(rename = "400gbase-x-cfp2")]
    Variant400gbaseXCfp2,
    #[serde(rename = "100gbase-x-cfp4")]
    Variant100gbaseXCfp4,
    #[serde(rename = "100gbase-x-cxp")]
    Variant100gbaseXCxp,
    #[serde(rename = "100gbase-x-cpak")]
    Variant100gbaseXCpak,
    #[serde(rename = "100gbase-x-dsfp")]
    Variant100gbaseXDsfp,
    #[serde(rename = "100gbase-x-sfpdd")]
    Variant100gbaseXSfpdd,
    #[serde(rename = "100gbase-x-qsfp28")]
    Variant100gbaseXQsfp28,
    #[serde(rename = "100gbase-x-qsfpdd")]
    Variant100gbaseXQsfpdd,
    #[serde(rename = "200gbase-x-qsfp56")]
    Variant200gbaseXQsfp56,
    #[serde(rename = "200gbase-x-qsfpdd")]
    Variant200gbaseXQsfpdd,
    #[serde(rename = "400gbase-x-qsfpdd")]
    Variant400gbaseXQsfpdd,
    #[serde(rename = "400gbase-x-osfp")]
    Variant400gbaseXOsfp,
    #[serde(rename = "400gbase-x-cdfp")]
    Variant400gbaseXCdfp,
    #[serde(rename = "400gbase-x-cfp8")]
    Variant400gbaseXCfp8,
    #[serde(rename = "800gbase-x-qsfpdd")]
    Variant800gbaseXQsfpdd,
    #[serde(rename = "800gbase-x-osfp")]
    Variant800gbaseXOsfp,
    #[serde(rename = "1000base-kx")]
    Variant1000baseKx,
    #[serde(rename = "10gbase-kr")]
    Variant10gbaseKr,
    #[serde(rename = "10gbase-kx4")]
    Variant10gbaseKx4,
    #[serde(rename = "25gbase-kr")]
    Variant25gbaseKr,
    #[serde(rename = "40gbase-kr4")]
    Variant40gbaseKr4,
    #[serde(rename = "50gbase-kr")]
    Variant50gbaseKr,
    #[serde(rename = "100gbase-kp4")]
    Variant100gbaseKp4,
    #[serde(rename = "100gbase-kr2")]
    Variant100gbaseKr2,
    #[serde(rename = "100gbase-kr4")]
    Variant100gbaseKr4,
    #[serde(rename = "ieee802.11a")]
    Ieee802Period11a,
    #[serde(rename = "ieee802.11g")]
    Ieee802Period11g,
    #[serde(rename = "ieee802.11n")]
    Ieee802Period11n,
    #[serde(rename = "ieee802.11ac")]
    Ieee802Period11ac,
    #[serde(rename = "ieee802.11ad")]
    Ieee802Period11ad,
    #[serde(rename = "ieee802.11ax")]
    Ieee802Period11ax,
    #[serde(rename = "ieee802.11ay")]
    Ieee802Period11ay,
    #[serde(rename = "ieee802.15.1")]
    Ieee802Period15Period1,
    #[serde(rename = "other-wireless")]
    OtherWireless,
    #[serde(rename = "gsm")]
    Gsm,
    #[serde(rename = "cdma")]
    Cdma,
    #[serde(rename = "lte")]
    Lte,
    #[serde(rename = "sonet-oc3")]
    SonetOc3,
    #[serde(rename = "sonet-oc12")]
    SonetOc12,
    #[serde(rename = "sonet-oc48")]
    SonetOc48,
    #[serde(rename = "sonet-oc192")]
    SonetOc192,
    #[serde(rename = "sonet-oc768")]
    SonetOc768,
    #[serde(rename = "sonet-oc1920")]
    SonetOc1920,
    #[serde(rename = "sonet-oc3840")]
    SonetOc3840,
    #[serde(rename = "1gfc-sfp")]
    Variant1gfcSfp,
    #[serde(rename = "2gfc-sfp")]
    Variant2gfcSfp,
    #[serde(rename = "4gfc-sfp")]
    Variant4gfcSfp,
    #[serde(rename = "8gfc-sfpp")]
    Variant8gfcSfpp,
    #[serde(rename = "16gfc-sfpp")]
    Variant16gfcSfpp,
    #[serde(rename = "32gfc-sfp28")]
    Variant32gfcSfp28,
    #[serde(rename = "64gfc-qsfpp")]
    Variant64gfcQsfpp,
    #[serde(rename = "128gfc-qsfp28")]
    Variant128gfcQsfp28,
    #[serde(rename = "infiniband-sdr")]
    InfinibandSdr,
    #[serde(rename = "infiniband-ddr")]
    InfinibandDdr,
    #[serde(rename = "infiniband-qdr")]
    InfinibandQdr,
    #[serde(rename = "infiniband-fdr10")]
    InfinibandFdr10,
    #[serde(rename = "infiniband-fdr")]
    InfinibandFdr,
    #[serde(rename = "infiniband-edr")]
    InfinibandEdr,
    #[serde(rename = "infiniband-hdr")]
    InfinibandHdr,
    #[serde(rename = "infiniband-ndr")]
    InfinibandNdr,
    #[serde(rename = "infiniband-xdr")]
    InfinibandXdr,
    #[serde(rename = "t1")]
    T1,
    #[serde(rename = "e1")]
    E1,
    #[serde(rename = "t3")]
    T3,
    #[serde(rename = "e3")]
    E3,
    #[serde(rename = "xdsl")]
    Xdsl,
    #[serde(rename = "docsis")]
    Docsis,
    #[serde(rename = "gpon")]
    Gpon,
    #[serde(rename = "xg-pon")]
    XgPon,
    #[serde(rename = "xgs-pon")]
    XgsPon,
    #[serde(rename = "ng-pon2")]
    NgPon2,
    #[serde(rename = "epon")]
    Epon,
    #[serde(rename = "10g-epon")]
    Variant10gEpon,
    #[serde(rename = "cisco-stackwise")]
    CiscoStackwise,
    #[serde(rename = "cisco-stackwise-plus")]
    CiscoStackwisePlus,
    #[serde(rename = "cisco-flexstack")]
    CiscoFlexstack,
    #[serde(rename = "cisco-flexstack-plus")]
    CiscoFlexstackPlus,
    #[serde(rename = "cisco-stackwise-80")]
    CiscoStackwise80,
    #[serde(rename = "cisco-stackwise-160")]
    CiscoStackwise160,
    #[serde(rename = "cisco-stackwise-320")]
    CiscoStackwise320,
    #[serde(rename = "cisco-stackwise-480")]
    CiscoStackwise480,
    #[serde(rename = "cisco-stackwise-1t")]
    CiscoStackwise1t,
    #[serde(rename = "juniper-vcp")]
    JuniperVcp,
    #[serde(rename = "extreme-summitstack")]
    ExtremeSummitstack,
    #[serde(rename = "extreme-summitstack-128")]
    ExtremeSummitstack128,
    #[serde(rename = "extreme-summitstack-256")]
    ExtremeSummitstack256,
    #[serde(rename = "extreme-summitstack-512")]
    ExtremeSummitstack512,
    #[serde(rename = "other")]
    Other,
}

impl Default for Type {
    fn default() -> Type {
        Self::Virtual
    }
}
/// * `pd` - PD * `pse` - PSE
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PoeMode {
    #[serde(rename = "pd")]
    Pd,
    #[serde(rename = "pse")]
    Pse,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "null")]
    Null,
}

impl Default for PoeMode {
    fn default() -> PoeMode {
        Self::Pd
    }
}
/// * `type1-ieee802.3af` - 802.3af (Type 1) * `type2-ieee802.3at` - 802.3at (Type 2) * `type3-ieee802.3bt` - 802.3bt (Type 3) * `type4-ieee802.3bt` - 802.3bt (Type 4) * `passive-24v-2pair` - Passive 24V (2-pair) * `passive-24v-4pair` - Passive 24V (4-pair) * `passive-48v-2pair` - Passive 48V (2-pair) * `passive-48v-4pair` - Passive 48V (4-pair)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PoeType {
    #[serde(rename = "type1-ieee802.3af")]
    Type1Ieee802Period3af,
    #[serde(rename = "type2-ieee802.3at")]
    Type2Ieee802Period3at,
    #[serde(rename = "type3-ieee802.3bt")]
    Type3Ieee802Period3bt,
    #[serde(rename = "type4-ieee802.3bt")]
    Type4Ieee802Period3bt,
    #[serde(rename = "passive-24v-2pair")]
    Passive24v2pair,
    #[serde(rename = "passive-24v-4pair")]
    Passive24v4pair,
    #[serde(rename = "passive-48v-2pair")]
    Passive48v2pair,
    #[serde(rename = "passive-48v-4pair")]
    Passive48v4pair,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "null")]
    Null,
}

impl Default for PoeType {
    fn default() -> PoeType {
        Self::Type1Ieee802Period3af
    }
}
/// * `ap` - Access point * `station` - Station
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RfRole {
    #[serde(rename = "ap")]
    Ap,
    #[serde(rename = "station")]
    Station,
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "null")]
    Null,
}

impl Default for RfRole {
    fn default() -> RfRole {
        Self::Ap
    }
}

