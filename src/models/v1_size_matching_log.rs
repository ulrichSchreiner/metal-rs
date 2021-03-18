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
pub struct V1SizeMatchingLog {
    #[serde(rename = "constraints")]
    pub constraints: Vec<crate::models::V1SizeConstraintMatchingLog>,
    #[serde(rename = "log")]
    pub log: String,
    #[serde(rename = "match")]
    pub _match: bool,
    #[serde(rename = "name")]
    pub name: String,
}

impl V1SizeMatchingLog {
    pub fn new(constraints: Vec<crate::models::V1SizeConstraintMatchingLog>, log: String, _match: bool, name: String) -> V1SizeMatchingLog {
        V1SizeMatchingLog {
            constraints,
            log,
            _match,
            name,
        }
    }
}

