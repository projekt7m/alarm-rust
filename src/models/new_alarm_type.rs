/*
 * Alarm Backend
 *
 * ## API for generating and tracking alarms  This is the API of what P7M mainly uses within the Telfas service.
 *
 * The version of the OpenAPI document: 0.3.0
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewAlarmType {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "sendCall")]
    pub send_call: bool,
    #[serde(rename = "sendSms")]
    pub send_sms: bool,
    #[serde(rename = "sendApp")]
    pub send_app: bool,
    #[serde(rename = "guidance")]
    pub guidance: models::AlarmGuidance,
    #[serde(rename = "callText")]
    pub call_text: String,
    #[serde(rename = "callbackText")]
    pub callback_text: String,
    #[serde(rename = "smsText")]
    pub sms_text: String,
}

impl NewAlarmType {
    pub fn new(label: String, send_call: bool, send_sms: bool, send_app: bool, guidance: models::AlarmGuidance, call_text: String, callback_text: String, sms_text: String) -> NewAlarmType {
        NewAlarmType {
            label,
            send_call,
            send_sms,
            send_app,
            guidance,
            call_text,
            callback_text,
            sms_text,
        }
    }
}

