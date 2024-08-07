/*
 * Alarm Backend
 *
 * ## API for generating and tracking alarms  This is the API of what P7M mainly uses within the Telfas service.
 *
 * The version of the OpenAPI document: 0.2.1
 * Contact: tech@p7m.de
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
    pub medium: crate::models::AlarmMedium,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "submissionTime", skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    #[serde(rename = "deliveryTime", skip_serializing_if = "Option::is_none")]
    pub delivery_time: Option<String>,
    #[serde(rename = "confirmation", skip_serializing_if = "Option::is_none")]
    pub confirmation: Option<Box<crate::models::AlarmConfirmation>>,
    #[serde(rename = "confirmationTime", skip_serializing_if = "Option::is_none")]
    pub confirmation_time: Option<String>,
    #[serde(rename = "lastChange")]
    pub last_change: String,
}

impl AlarmRecipient {
    pub fn new(alarm_recipient_id: String, tenant_id: String, alarm_id: String, account_id: String, medium: crate::models::AlarmMedium, address: String, last_change: String) -> AlarmRecipient {
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


