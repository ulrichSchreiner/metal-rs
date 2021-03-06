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
pub struct V1MachineIpmiReports {
    /// the partition id for the ipmi report
    #[serde(rename = "partitionid", skip_serializing_if = "Option::is_none")]
    pub partitionid: Option<String>,
    /// uuid to machinereport
    #[serde(rename = "reports", skip_serializing_if = "Option::is_none")]
    pub reports: Option<::std::collections::HashMap<String, crate::models::V1MachineIpmiReport>>,
}

impl V1MachineIpmiReports {
    pub fn new() -> V1MachineIpmiReports {
        V1MachineIpmiReports {
            partitionid: None,
            reports: None,
        }
    }
}


