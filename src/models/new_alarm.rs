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
pub struct NewAlarm {
    #[serde(rename = "alarmTypeId")]
    pub alarm_type_id: String,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "sendCall")]
    pub send_call: bool,
    #[serde(rename = "sendSms")]
    pub send_sms: bool,
    #[serde(rename = "sendApp")]
    pub send_app: bool,
    #[serde(rename = "guidance")]
    pub guidance: crate::models::AlarmGuidance,
    #[serde(rename = "triggerTime")]
    pub trigger_time: String,
    #[serde(rename = "triggerName", skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
    #[serde(rename = "triggerAccountId", skip_serializing_if = "Option::is_none")]
    pub trigger_account_id: Option<String>,
    #[serde(rename = "cancellationTime", skip_serializing_if = "Option::is_none")]
    pub cancellation_time: Option<String>,
    #[serde(rename = "cancellationAccountId", skip_serializing_if = "Option::is_none")]
    pub cancellation_account_id: Option<String>,
}

impl NewAlarm {
    pub fn new(alarm_type_id: String, label: String, send_call: bool, send_sms: bool, send_app: bool, guidance: crate::models::AlarmGuidance, trigger_time: String) -> NewAlarm {
        NewAlarm {
            alarm_type_id,
            label,
            send_call,
            send_sms,
            send_app,
            guidance,
            trigger_time,
            trigger_name: None,
            trigger_account_id: None,
            cancellation_time: None,
            cancellation_account_id: None,
        }
    }
}


