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
pub struct V1SwitchUpdateRequest {
    /// a description for this entity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// the unique ID of this entity
    #[serde(rename = "id")]
    pub id: String,
    /// the mode the switch currently has
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// a readable name for this entity
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// the id of the rack in which this switch is located
    #[serde(rename = "rack_id")]
    pub rack_id: String,
}

impl V1SwitchUpdateRequest {
    pub fn new(id: String, rack_id: String) -> V1SwitchUpdateRequest {
        V1SwitchUpdateRequest {
            description: None,
            id,
            mode: None,
            name: None,
            rack_id,
        }
    }
}


