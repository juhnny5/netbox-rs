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
pub struct PowerFeed {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "power_panel")]
    pub power_panel: Box<crate::models::NestedPowerPanel>,
    #[serde(rename = "rack", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rack: Option<Option<Box<crate::models::NestedRack>>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::Status5>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<crate::models::Type3>>,
    #[serde(rename = "supply", skip_serializing_if = "Option::is_none")]
    pub supply: Option<Box<crate::models::Supply>>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<Box<crate::models::Phase>>,
    #[serde(rename = "voltage", skip_serializing_if = "Option::is_none")]
    pub voltage: Option<i32>,
    #[serde(rename = "amperage", skip_serializing_if = "Option::is_none")]
    pub amperage: Option<i32>,
    /// Maximum permissible draw (percentage)
    #[serde(rename = "max_utilization", skip_serializing_if = "Option::is_none")]
    pub max_utilization: Option<i32>,
    /// Treat as if a cable is connected
    #[serde(rename = "mark_connected", skip_serializing_if = "Option::is_none")]
    pub mark_connected: Option<bool>,
    #[serde(rename = "cable", skip_serializing_if = "Option::is_none")]
    pub cable: Option<Box<crate::models::NestedCable>>,
    #[serde(rename = "cable_end", skip_serializing_if = "Option::is_none")]
    pub cable_end: Option<String>,
    ///  Return the appropriate serializer for the link termination model. 
    #[serde(rename = "link_peers", skip_serializing_if = "Option::is_none")]
    pub link_peers: Option<Vec<String>>,
    #[serde(rename = "link_peers_type", skip_serializing_if = "Option::is_none")]
    pub link_peers_type: Option<String>,
    ///  Return the appropriate serializer for the type of connected object. 
    #[serde(rename = "connected_endpoints", skip_serializing_if = "Option::is_none")]
    pub connected_endpoints: Option<Vec<String>>,
    #[serde(rename = "connected_endpoints_type", skip_serializing_if = "Option::is_none")]
    pub connected_endpoints_type: Option<String>,
    #[serde(rename = "connected_endpoints_reachable", skip_serializing_if = "Option::is_none")]
    pub connected_endpoints_reachable: Option<bool>,
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
    #[serde(rename = "_occupied", skip_serializing_if = "Option::is_none")]
    pub _occupied: Option<bool>,
}

impl PowerFeed {
    pub fn new(power_panel: crate::models::NestedPowerPanel, name: String) -> PowerFeed {
        PowerFeed {
            id: None,
            url: None,
            display: None,
            power_panel: Box::new(power_panel),
            rack: None,
            name,
            status: None,
            r#type: None,
            supply: None,
            phase: None,
            voltage: None,
            amperage: None,
            max_utilization: None,
            mark_connected: None,
            cable: None,
            cable_end: None,
            link_peers: None,
            link_peers_type: None,
            connected_endpoints: None,
            connected_endpoints_type: None,
            connected_endpoints_reachable: None,
            description: None,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            _occupied: None,
        }
    }
}


