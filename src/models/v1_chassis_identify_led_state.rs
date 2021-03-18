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
pub struct V1ChassisIdentifyLedState {
    /// a description why this chassis identify LED is in the given state
    #[serde(rename = "description")]
    pub description: String,
    /// the state of this chassis identify LED. empty means LED-OFF
    #[serde(rename = "value")]
    pub value: String,
}

impl V1ChassisIdentifyLedState {
    pub fn new(description: String, value: String) -> V1ChassisIdentifyLedState {
        V1ChassisIdentifyLedState {
            description,
            value,
        }
    }
}

