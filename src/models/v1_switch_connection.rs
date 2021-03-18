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
pub struct V1SwitchConnection {
    /// the machine id of the machine connected to the nic
    #[serde(rename = "machine_id", skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[serde(rename = "nic")]
    pub nic: crate::models::V1SwitchNic,
}

impl V1SwitchConnection {
    pub fn new(nic: crate::models::V1SwitchNic) -> V1SwitchConnection {
        V1SwitchConnection {
            machine_id: None,
            nic,
        }
    }
}

