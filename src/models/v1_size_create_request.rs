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
pub struct V1SizeCreateRequest {
    /// a list of constraints that defines this size
    #[serde(rename = "constraints")]
    pub constraints: Vec<crate::models::V1SizeConstraint>,
    /// a description for this entity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// the unique ID of this entity
    #[serde(rename = "id")]
    pub id: String,
    /// a readable name for this entity
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl V1SizeCreateRequest {
    pub fn new(constraints: Vec<crate::models::V1SizeConstraint>, id: String) -> V1SizeCreateRequest {
        V1SizeCreateRequest {
            constraints,
            description: None,
            id,
            name: None,
        }
    }
}


