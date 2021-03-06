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
pub struct RestStatus {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "status")]
    pub status: String,
}

impl RestStatus {
    pub fn new(message: String, status: String) -> RestStatus {
        RestStatus {
            message,
            status,
        }
    }
}


