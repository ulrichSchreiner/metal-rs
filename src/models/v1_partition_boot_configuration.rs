/*
 * metal-api
 *
 * API to manage and control plane resources like machines, switches, operating system images, machine sizes, networks, IP addresses and more
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: ulrich.schreiner@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// V1PartitionBootConfiguration : a partition has a distinct location in a data center, individual entities belong to a partition



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1PartitionBootConfiguration {
    /// the cmdline to the kernel for the boot image
    #[serde(rename = "commandline", skip_serializing_if = "Option::is_none")]
    pub commandline: Option<String>,
    /// the url to download the initrd for the boot image
    #[serde(rename = "imageurl", skip_serializing_if = "Option::is_none")]
    pub imageurl: Option<String>,
    /// the url to download the kernel for the boot image
    #[serde(rename = "kernelurl", skip_serializing_if = "Option::is_none")]
    pub kernelurl: Option<String>,
}

impl V1PartitionBootConfiguration {
    /// a partition has a distinct location in a data center, individual entities belong to a partition
    pub fn new() -> V1PartitionBootConfiguration {
        V1PartitionBootConfiguration {
            commandline: None,
            imageurl: None,
            kernelurl: None,
        }
    }
}


