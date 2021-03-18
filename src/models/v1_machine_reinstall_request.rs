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
pub struct V1MachineReinstallRequest {
    /// a description for this entity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// the unique ID of this entity
    #[serde(rename = "id")]
    pub id: String,
    /// the image id to be installed
    #[serde(rename = "imageid")]
    pub imageid: String,
    /// a readable name for this entity
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl V1MachineReinstallRequest {
    pub fn new(id: String, imageid: String) -> V1MachineReinstallRequest {
        V1MachineReinstallRequest {
            description: None,
            id,
            imageid,
            name: None,
        }
    }
}

