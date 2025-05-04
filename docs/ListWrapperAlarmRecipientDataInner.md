# ListWrapperAlarmRecipientDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alarm_recipient_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**alarm_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**recipient_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**medium** | [**models::AlarmMedium**](AlarmMedium.md) |  | 
**phone_number** | **String** |  | 
**submission_time** | Option<**String**> |  | [optional]
**delivery_time** | Option<**String**> |  | [optional]
**confirmation** | Option<[**models::AlarmConfirmation**](AlarmConfirmation.md)> |  | [optional]
**confirmation_time** | Option<**String**> |  | [optional]
**last_change** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


