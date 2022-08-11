# \AlarmRecipientApi

All URIs are relative to *https://alarm.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alarms_aid_recipients_get**](AlarmRecipientApi.md#alarms_aid_recipients_get) | **GET** /alarms/{aid}/recipients | 
[**alarms_aid_recipients_id_delete**](AlarmRecipientApi.md#alarms_aid_recipients_id_delete) | **DELETE** /alarms/{aid}/recipients/{id} | 
[**alarms_aid_recipients_id_get**](AlarmRecipientApi.md#alarms_aid_recipients_id_get) | **GET** /alarms/{aid}/recipients/{id} | 
[**alarms_aid_recipients_id_put**](AlarmRecipientApi.md#alarms_aid_recipients_id_put) | **PUT** /alarms/{aid}/recipients/{id} | 
[**alarms_aid_recipients_post**](AlarmRecipientApi.md#alarms_aid_recipients_post) | **POST** /alarms/{aid}/recipients | 



## alarms_aid_recipients_get

> crate::models::AlarmRecipientData alarms_aid_recipients_get(aid)


an alarm recipient tracks the progress of an alarm delivery to a recipient's device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** | alarm id | [required] |

### Return type

[**crate::models::AlarmRecipientData**](AlarmRecipientData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_aid_recipients_id_delete

> alarms_aid_recipients_id_delete(aid, id)


deletes the given alarm recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** | alarm id | [required] |
**id** | **String** | alarm recipient id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_aid_recipients_id_get

> crate::models::AlarmRecipient alarms_aid_recipients_id_get(aid, id)


Returns the given alarm recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** | alarm id | [required] |
**id** | **String** | alarm recipient id | [required] |

### Return type

[**crate::models::AlarmRecipient**](AlarmRecipient.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_aid_recipients_id_put

> crate::models::AlarmRecipient alarms_aid_recipients_id_put(aid, id, new_alarm_recipient)


updates the given alarm recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** | alarm id | [required] |
**id** | **String** | alarm recipient id | [required] |
**new_alarm_recipient** | [**NewAlarmRecipient**](NewAlarmRecipient.md) | the updated alarm recipient | [required] |

### Return type

[**crate::models::AlarmRecipient**](AlarmRecipient.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_aid_recipients_post

> crate::models::AlarmRecipient alarms_aid_recipients_post(aid, new_alarm_recipient)


creates a new alarm recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** | alarm id | [required] |
**new_alarm_recipient** | [**NewAlarmRecipient**](NewAlarmRecipient.md) | the alarm recipient to be created | [required] |

### Return type

[**crate::models::AlarmRecipient**](AlarmRecipient.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

