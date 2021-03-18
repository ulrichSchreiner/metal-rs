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
pub struct V1ServerCapacity {
    /// allocated servers with this size
    #[serde(rename = "allocated")]
    pub allocated: i64,
    /// servers with issues with this size
    #[serde(rename = "faulty")]
    pub faulty: i64,
    /// servers with issues with this size
    #[serde(rename = "faultymachines")]
    pub faultymachines: Vec<String>,
    /// free servers with this size
    #[serde(rename = "free")]
    pub free: i64,
    /// servers neither free, allocated or faulty with this size
    #[serde(rename = "other")]
    pub other: i64,
    /// servers neither free, allocated or faulty with this size
    #[serde(rename = "othermachines")]
    pub othermachines: Vec<String>,
    /// the size of the server
    #[serde(rename = "size")]
    pub size: String,
    /// total amount of servers with this size
    #[serde(rename = "total")]
    pub total: i64,
}

impl V1ServerCapacity {
    pub fn new(allocated: i64, faulty: i64, faultymachines: Vec<String>, free: i64, other: i64, othermachines: Vec<String>, size: String, total: i64) -> V1ServerCapacity {
        V1ServerCapacity {
            allocated,
            faulty,
            faultymachines,
            free,
            other,
            othermachines,
            size,
            total,
        }
    }
}


