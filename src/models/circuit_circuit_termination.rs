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
pub struct CircuitCircuitTermination {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "site", deserialize_with = "Option::deserialize")]
    pub site: Option<Box<crate::models::NestedSite>>,
    #[serde(rename = "provider_network", deserialize_with = "Option::deserialize")]
    pub provider_network: Option<Box<crate::models::NestedProviderNetwork>>,
    #[serde(rename = "port_speed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub port_speed: Option<Option<i32>>,
    /// Upstream speed, if different from port speed
    #[serde(rename = "upstream_speed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub upstream_speed: Option<Option<i32>>,
    #[serde(rename = "xconnect_id", skip_serializing_if = "Option::is_none")]
    pub xconnect_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CircuitCircuitTermination {
    pub fn new(site: Option<crate::models::NestedSite>, provider_network: Option<crate::models::NestedProviderNetwork>) -> CircuitCircuitTermination {
        CircuitCircuitTermination {
            id: None,
            url: None,
            display: None,
            site: if let Some(x) = site {Some(Box::new(x))} else {None},
            provider_network: if let Some(x) = provider_network {Some(Box::new(x))} else {None},
            port_speed: None,
            upstream_speed: None,
            xconnect_id: None,
            description: None,
        }
    }
}


