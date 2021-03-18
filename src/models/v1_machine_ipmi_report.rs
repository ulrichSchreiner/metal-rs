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
pub struct V1MachineIpmiReport {
    #[serde(rename = "BIOSVersion")]
    pub bios_version: String,
    #[serde(rename = "BMCIp")]
    pub bmcip: String,
    #[serde(rename = "BMCVersion")]
    pub bmc_version: String,
    #[serde(rename = "FRU")]
    pub FRU: crate::models::V1MachineFru,
}

impl V1MachineIpmiReport {
    pub fn new(bios_version: String, bmcip: String, bmc_version: String, FRU: crate::models::V1MachineFru) -> V1MachineIpmiReport {
        V1MachineIpmiReport {
            bios_version,
            bmcip,
            bmc_version,
            FRU,
        }
    }
}

