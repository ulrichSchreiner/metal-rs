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
pub struct V1ImageResponse {
    /// the last changed timestamp of this entity
    #[serde(rename = "changed", skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
    /// classification of this image
    #[serde(rename = "classification", skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// the creation time of this entity
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// a description for this entity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// expirationDate of this image
    #[serde(rename = "expirationDate")]
    pub expiration_date: String,
    /// features of this image
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    /// the unique ID of this entity
    #[serde(rename = "id")]
    pub id: String,
    /// a readable name for this entity
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// the url of this image
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// machines where this image is in use
    #[serde(rename = "usedby", skip_serializing_if = "Option::is_none")]
    pub usedby: Option<Vec<String>>,
}

impl V1ImageResponse {
    pub fn new(expiration_date: String, id: String) -> V1ImageResponse {
        V1ImageResponse {
            changed: None,
            classification: None,
            created: None,
            description: None,
            expiration_date,
            features: None,
            id,
            name: None,
            url: None,
            usedby: None,
        }
    }
}


