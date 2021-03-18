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
pub struct V1NetworkUsage {
    /// the total available IPs
    #[serde(rename = "available_ips")]
    pub available_ips: i64,
    /// the total available Prefixes
    #[serde(rename = "available_prefixes")]
    pub available_prefixes: i64,
    /// the total used IPs
    #[serde(rename = "used_ips")]
    pub used_ips: i64,
    /// the total used Prefixes
    #[serde(rename = "used_prefixes")]
    pub used_prefixes: i64,
}

impl V1NetworkUsage {
    pub fn new(available_ips: i64, available_prefixes: i64, used_ips: i64, used_prefixes: i64) -> V1NetworkUsage {
        V1NetworkUsage {
            available_ips,
            available_prefixes,
            used_ips,
            used_prefixes,
        }
    }
}

