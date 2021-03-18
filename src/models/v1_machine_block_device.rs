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
pub struct V1MachineBlockDevice {
    /// the name of this block device
    #[serde(rename = "name")]
    pub name: String,
    /// the partitions of this disk
    #[serde(rename = "partitions", skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<crate::models::V1MachineDiskPartition>>,
    /// whether this disk has the OS installed
    #[serde(rename = "primary")]
    pub primary: bool,
    /// the size of this block device
    #[serde(rename = "size")]
    pub size: i64,
}

impl V1MachineBlockDevice {
    pub fn new(name: String, primary: bool, size: i64) -> V1MachineBlockDevice {
        V1MachineBlockDevice {
            name,
            partitions: None,
            primary,
            size,
        }
    }
}


