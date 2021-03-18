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
pub struct V1QuotaSet {
    #[serde(rename = "cluster", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<crate::models::V1Quota>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<crate::models::V1Quota>,
    #[serde(rename = "machine", skip_serializing_if = "Option::is_none")]
    pub machine: Option<crate::models::V1Quota>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<crate::models::V1Quota>,
}

impl V1QuotaSet {
    pub fn new() -> V1QuotaSet {
        V1QuotaSet {
            cluster: None,
            ip: None,
            machine: None,
            project: None,
        }
    }
}


