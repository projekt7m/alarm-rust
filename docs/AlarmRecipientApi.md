# \AlarmRecipientApi

All URIs are relative to *https://alarm.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_alarms_recipients_id**](AlarmRecipientApi.md#delete_alarms_recipients_id) | **DELETE** /alarms/{aid}/recipients/{id} | Delete a single alarm recipient by its ID
[**get_alarms_recipients**](AlarmRecipientApi.md#get_alarms_recipients) | **GET** /alarms/{aid}/recipients | Get the list of receipients of an alarm
[**get_alarms_recipients_id**](AlarmRecipientApi.md#get_alarms_recipients_id) | **GET** /alarms/{aid}/recipients/{id} | Get a single recipient of an alarm by its ID
[**post_alarm_recipients**](AlarmRecipientApi.md#post_alarm_recipients) | **POST** /alarms/{aid}/recipients | Add a new recipient to an existing alarm
[**put_alarms_recipients_id**](AlarmRecipientApi.md#put_alarms_recipients_id) | **PUT** /alarms/{aid}/recipients/{id} | Update an existing recipient of an alarm



## delete_alarms_recipients_id

> delete_alarms_recipients_id(aid, id)
Delete a single alarm recipient by its ID

Delete a single alarm recipient by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** | ID of the alarm | [required] |
**id** | **String** | ID of the alarm recipient | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alarms_recipients

> crate::models::AlarmRecipientData get_alarms_recipients(aid)
Get the list of receipients of an alarm

Get the list of receipients of an alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** | ID of the alarm | [required] |

### Return type

[**crate::models::AlarmRecipientData**](AlarmRecipientData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alarms_recipients_id

> crate::models::AlarmRecipient get_alarms_recipients_id(aid, id)
Get a single recipient of an alarm by its ID

Get a single recipient of an alarm by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** | ID of the alarm | [required] |
**id** | **String** | ID of the alarm recipient | [required] |

### Return type

[**crate::models::AlarmRecipient**](AlarmRecipient.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_alarm_recipients

> crate::models::AlarmRecipient post_alarm_recipients(aid, new_alarm_recipient)
Add a new recipient to an existing alarm

Add a new recipient to an existing alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** | ID of the alarm | [required] |
**new_alarm_recipient** | [**NewAlarmRecipient**](NewAlarmRecipient.md) | The recipient to be added | [required] |

### Return type

[**crate::models::AlarmRecipient**](AlarmRecipient.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_alarms_recipients_id

> crate::models::AlarmRecipient put_alarms_recipients_id(aid, id, new_alarm_recipient)
Update an existing recipient of an alarm

Update an existing recipient of an alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** | ID of the alarm | [required] |
**id** | **String** | ID of the alarm recipient | [required] |
**new_alarm_recipient** | [**NewAlarmRecipient**](NewAlarmRecipient.md) | The updated alarm recipient | [required] |

### Return type

[**crate::models::AlarmRecipient**](AlarmRecipient.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

