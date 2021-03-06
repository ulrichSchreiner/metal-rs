/*
 * metal-api
 *
 * API to manage and control plane resources like machines, switches, operating system images, machine sizes, networks, IP addresses and more
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: ulrich.schreiner@gmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1SwitchResponse {
    /// the last changed timestamp of this entity
    #[serde(rename = "changed", skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
    /// a connection between a switch port and a machine
    #[serde(rename = "connections")]
    pub connections: Vec<crate::models::V1SwitchConnection>,
    /// the creation time of this entity
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// a description for this entity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// the unique ID of this entity
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "last_sync", skip_serializing_if = "Option::is_none")]
    pub last_sync: Option<crate::models::V1SwitchSync>,
    #[serde(rename = "last_sync_error", skip_serializing_if = "Option::is_none")]
    pub last_sync_error: Option<crate::models::V1SwitchSync>,
    /// the mode the switch currently has
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// a readable name for this entity
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// the list of network interfaces on the switch
    #[serde(rename = "nics")]
    pub nics: Vec<crate::models::V1SwitchNic>,
    #[serde(rename = "partition")]
    pub partition: crate::models::V1PartitionResponse,
    /// the id of the rack in which this switch is located
    #[serde(rename = "rack_id")]
    pub rack_id: String,
}

impl V1SwitchResponse {
    pub fn new(connections: Vec<crate::models::V1SwitchConnection>, id: String, nics: Vec<crate::models::V1SwitchNic>, partition: crate::models::V1PartitionResponse, rack_id: String) -> V1SwitchResponse {
        V1SwitchResponse {
            changed: None,
            connections,
            created: None,
            description: None,
            id,
            last_sync: None,
            last_sync_error: None,
            mode: None,
            name: None,
            nics,
            partition,
            rack_id,
        }
    }
}


