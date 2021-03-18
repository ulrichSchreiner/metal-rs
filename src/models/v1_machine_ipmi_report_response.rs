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
pub struct V1MachineIpmiReportResponse {
    /// the machine uuids that triggered a creation of a machine entity
    #[serde(rename = "created")]
    pub created: Vec<String>,
    /// the machine uuids that triggered an update of ipmi data
    #[serde(rename = "updated")]
    pub updated: Vec<String>,
}

impl V1MachineIpmiReportResponse {
    pub fn new(created: Vec<String>, updated: Vec<String>) -> V1MachineIpmiReportResponse {
        V1MachineIpmiReportResponse {
            created,
            updated,
        }
    }
}

