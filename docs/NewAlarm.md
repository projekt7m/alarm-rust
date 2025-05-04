# NewAlarm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alarm_type_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**label** | **String** |  | 
**send_call** | **bool** |  | 
**send_sms** | **bool** |  | 
**send_app** | **bool** |  | 
**guidance** | [**models::AlarmGuidance**](AlarmGuidance.md) |  | 
**trigger_time** | **String** |  | 
**trigger_name** | Option<**String**> |  | [optional]
**trigger_account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**cancellation_time** | Option<**String**> |  | [optional]
**cancellation_account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


