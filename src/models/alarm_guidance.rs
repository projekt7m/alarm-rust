/*
 * Alarm Backend
 *
 * ## API for generating and tracking alarms  This is the API of what P7M mainly uses within the Telfas service.
 *
 * The version of the OpenAPI document: 0.2.1
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlarmGuidance {
    #[serde(rename = "EVACUATE")]
    EVACUATE,
    #[serde(rename = "GO_TO_SAFE_PLACE")]
    GOTOSAFEPLACE,

}

impl ToString for AlarmGuidance {
    fn to_string(&self) -> String {
        match self {
            Self::EVACUATE => String::from("EVACUATE"),
            Self::GOTOSAFEPLACE => String::from("GO_TO_SAFE_PLACE"),
        }
    }
}

impl Default for AlarmGuidance {
    fn default() -> AlarmGuidance {
        Self::EVACUATE
    }
}




