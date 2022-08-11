/*
 * Alarm Backend
 *
 * # API for generating and tracking alarms  This is the API of what P7M mainly uses within the Telfas service.  For most endpoints, the caller has to be authenticated with the system and provide a JWT token in the Authorization header of the HTTP request. If your interacting with this API using the Swagger interface, you need to set the JWT token by clicking on the **Authorize** button on the right side of the header. As the value don't forget that the Authorization header starts with the fixed value `Bearer` followed by a space followed by the actual JWT token value.  If anything is unclear or you found a bug in the documentation, please contact <tech@p7m.de>. 
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AlarmRecipient {
    #[serde(rename = "alarmRecipientId")]
    pub alarm_recipient_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "alarmId")]
    pub alarm_id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "medium")]
    pub medium: Medium,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "submissionTime", skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    #[serde(rename = "deliveryTime", skip_serializing_if = "Option::is_none")]
    pub delivery_time: Option<String>,
    #[serde(rename = "confirmation", skip_serializing_if = "Option::is_none")]
    pub confirmation: Option<Confirmation>,
    #[serde(rename = "confirmationTime", skip_serializing_if = "Option::is_none")]
    pub confirmation_time: Option<String>,
    #[serde(rename = "lastChange")]
    pub last_change: String,
}

impl AlarmRecipient {
    pub fn new(alarm_recipient_id: String, tenant_id: String, alarm_id: String, account_id: String, medium: Medium, address: String, last_change: String) -> AlarmRecipient {
        AlarmRecipient {
            alarm_recipient_id,
            tenant_id,
            alarm_id,
            account_id,
            medium,
            address,
            submission_time: None,
            delivery_time: None,
            confirmation: None,
            confirmation_time: None,
            last_change,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Medium {
    #[serde(rename = "CALL")]
    CALL,
    #[serde(rename = "SMS")]
    SMS,
    #[serde(rename = "APP")]
    APP,
}

impl Default for Medium {
    fn default() -> Medium {
        Self::CALL
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Confirmation {
    #[serde(rename = "OFF_SITE")]
    OFFSITE,
    #[serde(rename = "EVACUATED")]
    EVACUATED,
    #[serde(rename = "SAFE_SPOT")]
    SAFESPOT,
}

impl Default for Confirmation {
    fn default() -> Confirmation {
        Self::OFFSITE
    }
}
