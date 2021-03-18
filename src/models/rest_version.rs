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
pub struct RestVersion {
    #[serde(rename = "builddate")]
    pub builddate: String,
    #[serde(rename = "gitsha1")]
    pub gitsha1: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "revision")]
    pub revision: String,
    #[serde(rename = "version")]
    pub version: String,
}

impl RestVersion {
    pub fn new(builddate: String, gitsha1: String, name: String, revision: String, version: String) -> RestVersion {
        RestVersion {
            builddate,
            gitsha1,
            name,
            revision,
            version,
        }
    }
}

