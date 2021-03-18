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
pub struct V1SwitchNic {
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<crate::models::V1BgpFilter>,
    /// the mac address of this network interface
    #[serde(rename = "mac")]
    pub mac: String,
    /// the name of this network interface
    #[serde(rename = "name")]
    pub name: String,
    /// the vrf this network interface is part of
    #[serde(rename = "vrf", skip_serializing_if = "Option::is_none")]
    pub vrf: Option<String>,
}

impl V1SwitchNic {
    pub fn new(mac: String, name: String) -> V1SwitchNic {
        V1SwitchNic {
            filter: None,
            mac,
            name,
            vrf: None,
        }
    }
}


