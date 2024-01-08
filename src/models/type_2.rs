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
pub struct Type2 {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Type2 {
    pub fn new(label: Label, value: Value) -> Type2 {
        Type2 {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Virtual")]
    Virtual,
    #[serde(rename = "Bridge")]
    Bridge,
    #[serde(rename = "Link Aggregation Group (LAG)")]
    LinkAggregationGroupLeftParenthesisLagRightParenthesis,
    #[serde(rename = "100BASE-FX (10/100ME FIBER)")]
    Variant100BaseFxLeftParenthesis10Slash100MeFiberRightParenthesis,
    #[serde(rename = "100BASE-LFX (10/100ME FIBER)")]
    Variant100BaseLfxLeftParenthesis10Slash100MeFiberRightParenthesis,
    #[serde(rename = "100BASE-TX (10/100ME)")]
    Variant100BaseTxLeftParenthesis10Slash100MeRightParenthesis,
    #[serde(rename = "100BASE-T1 (10/100ME Single Pair)")]
    Variant100BaseT1LeftParenthesis10Slash100MeSinglePairRightParenthesis,
    #[serde(rename = "1000BASE-T (1GE)")]
    Variant1000BaseTLeftParenthesis1GeRightParenthesis,
    #[serde(rename = "2.5GBASE-T (2.5GE)")]
    Variant2Period5GbaseTLeftParenthesis2Period5GeRightParenthesis,
    #[serde(rename = "5GBASE-T (5GE)")]
    Variant5GbaseTLeftParenthesis5GeRightParenthesis,
    #[serde(rename = "10GBASE-T (10GE)")]
    Variant10GbaseTLeftParenthesis10GeRightParenthesis,
    #[serde(rename = "10GBASE-CX4 (10GE)")]
    Variant10GbaseCx4LeftParenthesis10GeRightParenthesis,
    #[serde(rename = "GBIC (1GE)")]
    GbicLeftParenthesis1GeRightParenthesis,
    #[serde(rename = "SFP (1GE)")]
    SfpLeftParenthesis1GeRightParenthesis,
    #[serde(rename = "SFP+ (10GE)")]
    SfpPlusLeftParenthesis10GeRightParenthesis,
    #[serde(rename = "XFP (10GE)")]
    XfpLeftParenthesis10GeRightParenthesis,
    #[serde(rename = "XENPAK (10GE)")]
    XenpakLeftParenthesis10GeRightParenthesis,
    #[serde(rename = "X2 (10GE)")]
    X2LeftParenthesis10GeRightParenthesis,
    #[serde(rename = "SFP28 (25GE)")]
    Sfp28LeftParenthesis25GeRightParenthesis,
    #[serde(rename = "SFP56 (50GE)")]
    Sfp56LeftParenthesis50GeRightParenthesis,
    #[serde(rename = "QSFP+ (40GE)")]
    QsfpPlusLeftParenthesis40GeRightParenthesis,
    #[serde(rename = "QSFP28 (50GE)")]
    Qsfp28LeftParenthesis50GeRightParenthesis,
    #[serde(rename = "CFP (100GE)")]
    CfpLeftParenthesis100GeRightParenthesis,
    #[serde(rename = "CFP2 (100GE)")]
    Cfp2LeftParenthesis100GeRightParenthesis,
    #[serde(rename = "CFP2 (200GE)")]
    Cfp2LeftParenthesis200GeRightParenthesis,
    #[serde(rename = "CFP4 (100GE)")]
    Cfp4LeftParenthesis100GeRightParenthesis,
    #[serde(rename = "Cisco CPAK (100GE)")]
    CiscoCpakLeftParenthesis100GeRightParenthesis,
    #[serde(rename = "QSFP28 (100GE)")]
    Qsfp28LeftParenthesis100GeRightParenthesis,
    #[serde(rename = "QSFP56 (200GE)")]
    Qsfp56LeftParenthesis200GeRightParenthesis,
    #[serde(rename = "QSFP-DD (400GE)")]
    QsfpDdLeftParenthesis400GeRightParenthesis,
    #[serde(rename = "OSFP (400GE)")]
    OsfpLeftParenthesis400GeRightParenthesis,
    #[serde(rename = "QSFP-DD (800GE)")]
    QsfpDdLeftParenthesis800GeRightParenthesis,
    #[serde(rename = "OSFP (800GE)")]
    OsfpLeftParenthesis800GeRightParenthesis,
    #[serde(rename = "1000BASE-KX (1GE)")]
    Variant1000BaseKxLeftParenthesis1GeRightParenthesis,
    #[serde(rename = "10GBASE-KR (10GE)")]
    Variant10GbaseKrLeftParenthesis10GeRightParenthesis,
    #[serde(rename = "10GBASE-KX4 (10GE)")]
    Variant10GbaseKx4LeftParenthesis10GeRightParenthesis,
    #[serde(rename = "25GBASE-KR (25GE)")]
    Variant25GbaseKrLeftParenthesis25GeRightParenthesis,
    #[serde(rename = "40GBASE-KR4 (40GE)")]
    Variant40GbaseKr4LeftParenthesis40GeRightParenthesis,
    #[serde(rename = "50GBASE-KR (50GE)")]
    Variant50GbaseKrLeftParenthesis50GeRightParenthesis,
    #[serde(rename = "100GBASE-KP4 (100GE)")]
    Variant100GbaseKp4LeftParenthesis100GeRightParenthesis,
    #[serde(rename = "100GBASE-KR2 (100GE)")]
    Variant100GbaseKr2LeftParenthesis100GeRightParenthesis,
    #[serde(rename = "100GBASE-KR4 (100GE)")]
    Variant100GbaseKr4LeftParenthesis100GeRightParenthesis,
    #[serde(rename = "IEEE 802.11a")]
    Ieee802Period11a,
    #[serde(rename = "IEEE 802.11b/g")]
    Ieee802Period11bSlashG,
    #[serde(rename = "IEEE 802.11n")]
    Ieee802Period11n,
    #[serde(rename = "IEEE 802.11ac")]
    Ieee802Period11ac,
    #[serde(rename = "IEEE 802.11ad")]
    Ieee802Period11ad,
    #[serde(rename = "IEEE 802.11ax")]
    Ieee802Period11ax,
    #[serde(rename = "IEEE 802.11ay")]
    Ieee802Period11ay,
    #[serde(rename = "IEEE 802.15.1 (Bluetooth)")]
    Ieee802Period15Period1LeftParenthesisBluetoothRightParenthesis,
    #[serde(rename = "Other (Wireless)")]
    OtherLeftParenthesisWirelessRightParenthesis,
    #[serde(rename = "GSM")]
    Gsm,
    #[serde(rename = "CDMA")]
    Cdma,
    #[serde(rename = "LTE")]
    Lte,
    #[serde(rename = "OC-3/STM-1")]
    Oc3SlashStm1,
    #[serde(rename = "OC-12/STM-4")]
    Oc12SlashStm4,
    #[serde(rename = "OC-48/STM-16")]
    Oc48SlashStm16,
    #[serde(rename = "OC-192/STM-64")]
    Oc192SlashStm64,
    #[serde(rename = "OC-768/STM-256")]
    Oc768SlashStm256,
    #[serde(rename = "OC-1920/STM-640")]
    Oc1920SlashStm640,
    #[serde(rename = "OC-3840/STM-1234")]
    Oc3840SlashStm1234,
    #[serde(rename = "SFP (1GFC)")]
    SfpLeftParenthesis1GfcRightParenthesis,
    #[serde(rename = "SFP (2GFC)")]
    SfpLeftParenthesis2GfcRightParenthesis,
    #[serde(rename = "SFP (4GFC)")]
    SfpLeftParenthesis4GfcRightParenthesis,
    #[serde(rename = "SFP+ (8GFC)")]
    SfpPlusLeftParenthesis8GfcRightParenthesis,
    #[serde(rename = "SFP+ (16GFC)")]
    SfpPlusLeftParenthesis16GfcRightParenthesis,
    #[serde(rename = "SFP28 (32GFC)")]
    Sfp28LeftParenthesis32GfcRightParenthesis,
    #[serde(rename = "QSFP+ (64GFC)")]
    QsfpPlusLeftParenthesis64GfcRightParenthesis,
    #[serde(rename = "QSFP28 (128GFC)")]
    Qsfp28LeftParenthesis128GfcRightParenthesis,
    #[serde(rename = "SDR (2 Gbps)")]
    SdrLeftParenthesis2GbpsRightParenthesis,
    #[serde(rename = "DDR (4 Gbps)")]
    DdrLeftParenthesis4GbpsRightParenthesis,
    #[serde(rename = "QDR (8 Gbps)")]
    QdrLeftParenthesis8GbpsRightParenthesis,
    #[serde(rename = "FDR10 (10 Gbps)")]
    Fdr10LeftParenthesis10GbpsRightParenthesis,
    #[serde(rename = "FDR (13.5 Gbps)")]
    FdrLeftParenthesis13Period5GbpsRightParenthesis,
    #[serde(rename = "EDR (25 Gbps)")]
    EdrLeftParenthesis25GbpsRightParenthesis,
    #[serde(rename = "HDR (50 Gbps)")]
    HdrLeftParenthesis50GbpsRightParenthesis,
    #[serde(rename = "NDR (100 Gbps)")]
    NdrLeftParenthesis100GbpsRightParenthesis,
    #[serde(rename = "XDR (250 Gbps)")]
    XdrLeftParenthesis250GbpsRightParenthesis,
    #[serde(rename = "T1 (1.544 Mbps)")]
    T1LeftParenthesis1Period544MbpsRightParenthesis,
    #[serde(rename = "E1 (2.048 Mbps)")]
    E1LeftParenthesis2Period048MbpsRightParenthesis,
    #[serde(rename = "T3 (45 Mbps)")]
    T3LeftParenthesis45MbpsRightParenthesis,
    #[serde(rename = "E3 (34 Mbps)")]
    E3LeftParenthesis34MbpsRightParenthesis,
    #[serde(rename = "xDSL")]
    XDsl,
    #[serde(rename = "DOCSIS")]
    Docsis,
    #[serde(rename = "GPON (2.5 Gbps / 1.25 Gps)")]
    GponLeftParenthesis2Period5GbpsSlash1Period25GpsRightParenthesis,
    #[serde(rename = "XG-PON (10 Gbps / 2.5 Gbps)")]
    XgPonLeftParenthesis10GbpsSlash2Period5GbpsRightParenthesis,
    #[serde(rename = "XGS-PON (10 Gbps)")]
    XgsPonLeftParenthesis10GbpsRightParenthesis,
    #[serde(rename = "NG-PON2 (TWDM-PON) (4x10 Gbps)")]
    NgPon2LeftParenthesisTwdmPonRightParenthesisLeftParenthesis4x10GbpsRightParenthesis,
    #[serde(rename = "EPON (1 Gbps)")]
    EponLeftParenthesis1GbpsRightParenthesis,
    #[serde(rename = "10G-EPON (10 Gbps)")]
    Variant10GEponLeftParenthesis10GbpsRightParenthesis,
    #[serde(rename = "Cisco StackWise")]
    CiscoStackWise,
    #[serde(rename = "Cisco StackWise Plus")]
    CiscoStackWisePlus,
    #[serde(rename = "Cisco FlexStack")]
    CiscoFlexStack,
    #[serde(rename = "Cisco FlexStack Plus")]
    CiscoFlexStackPlus,
    #[serde(rename = "Cisco StackWise-80")]
    CiscoStackWise80,
    #[serde(rename = "Cisco StackWise-160")]
    CiscoStackWise160,
    #[serde(rename = "Cisco StackWise-320")]
    CiscoStackWise320,
    #[serde(rename = "Cisco StackWise-480")]
    CiscoStackWise480,
    #[serde(rename = "Juniper VCP")]
    JuniperVcp,
    #[serde(rename = "Extreme SummitStack")]
    ExtremeSummitStack,
    #[serde(rename = "Extreme SummitStack-128")]
    ExtremeSummitStack128,
    #[serde(rename = "Extreme SummitStack-256")]
    ExtremeSummitStack256,
    #[serde(rename = "Extreme SummitStack-512")]
    ExtremeSummitStack512,
    #[serde(rename = "Other")]
    Other,
}

impl Default for Label {
    fn default() -> Label {
        Self::Virtual
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
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
    #[serde(rename = "100gbase-x-cfp4")]
    Variant100gbaseXCfp4,
    #[serde(rename = "100gbase-x-cpak")]
    Variant100gbaseXCpak,
    #[serde(rename = "100gbase-x-qsfp28")]
    Variant100gbaseXQsfp28,
    #[serde(rename = "200gbase-x-qsfp56")]
    Variant200gbaseXQsfp56,
    #[serde(rename = "400gbase-x-qsfpdd")]
    Variant400gbaseXQsfpdd,
    #[serde(rename = "400gbase-x-osfp")]
    Variant400gbaseXOsfp,
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

impl Default for Value {
    fn default() -> Value {
        Self::Virtual
    }
}

