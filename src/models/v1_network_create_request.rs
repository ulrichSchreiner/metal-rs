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
pub struct V1NetworkCreateRequest {
    /// a description for this entity
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// the destination prefixes of this network
    #[serde(rename = "destinationprefixes")]
    pub destinationprefixes: Vec<String>,
    /// the unique ID of this entity, auto-generated if left empty
    #[serde(rename = "id")]
    pub id: String,
    /// free labels that you associate with this network.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// a readable name for this entity
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// if set to true, packets leaving this network get masqueraded behind interface ip
    #[serde(rename = "nat")]
    pub nat: bool,
    /// the id of the parent network
    #[serde(rename = "parentnetworkid", skip_serializing_if = "Option::is_none")]
    pub parentnetworkid: Option<String>,
    /// the partition this network belongs to
    #[serde(rename = "partitionid", skip_serializing_if = "Option::is_none")]
    pub partitionid: Option<String>,
    /// the prefixes of this network
    #[serde(rename = "prefixes")]
    pub prefixes: Vec<String>,
    /// if set to true, this network will serve as a partition's super network for the internal machine networks,there can only be one privatesuper network per partition
    #[serde(rename = "privatesuper")]
    pub privatesuper: bool,
    /// the project id this network belongs to, can be empty if globally available
    #[serde(rename = "projectid", skip_serializing_if = "Option::is_none")]
    pub projectid: Option<String>,
    /// marks a network as shareable.
    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    /// if set to true, this network can be used for underlay communication
    #[serde(rename = "underlay")]
    pub underlay: bool,
    /// the vrf this network is associated with
    #[serde(rename = "vrf", skip_serializing_if = "Option::is_none")]
    pub vrf: Option<i64>,
    /// if set to true, given vrf can be used by multiple networks, which is sometimes useful for network partioning (default: false)
    #[serde(rename = "vrfshared", skip_serializing_if = "Option::is_none")]
    pub vrfshared: Option<bool>,
}

impl V1NetworkCreateRequest {
    pub fn new(destinationprefixes: Vec<String>, id: String, nat: bool, prefixes: Vec<String>, privatesuper: bool, underlay: bool) -> V1NetworkCreateRequest {
        V1NetworkCreateRequest {
            description: None,
            destinationprefixes,
            id,
            labels: None,
            name: None,
            nat,
            parentnetworkid: None,
            partitionid: None,
            prefixes,
            privatesuper,
            projectid: None,
            shared: None,
            underlay,
            vrf: None,
            vrfshared: None,
        }
    }
}


