/*
 * metal-api
 *
 * API to manage and control plane resources like machines, switches, operating system images, machine sizes, networks, IP addresses and more
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: ulrich.schreiner@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// V1SizeConstraint : a machine matches to a size in order to make them easier to categorize



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1SizeConstraint {
    /// the maximum value of the constraint
    #[serde(rename = "max")]
    pub max: i64,
    /// the minimum value of the constraint
    #[serde(rename = "min")]
    pub min: i64,
    /// the type of the constraint
    #[serde(rename = "type")]
    pub _type: Type,
}

impl V1SizeConstraint {
    /// a machine matches to a size in order to make them easier to categorize
    pub fn new(max: i64, min: i64, _type: Type) -> V1SizeConstraint {
        V1SizeConstraint {
            max,
            min,
            _type,
        }
    }
}

/// the type of the constraint
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "cores")]
    Cores,
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "storage")]
    Storage,
}
