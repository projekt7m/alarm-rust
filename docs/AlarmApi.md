# \AlarmApi

All URIs are relative to *https://alarm.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_alarms_id**](AlarmApi.md#delete_alarms_id) | **DELETE** /alarms/{alarm_id} | Delete an alarm by its ID
[**delete_alarms_recipients_id**](AlarmApi.md#delete_alarms_recipients_id) | **DELETE** /alarms/{alarm_id}/recipients/{alarm_recipient_id} | Delete a single alarm recipient by its ID
[**get_alarms**](AlarmApi.md#get_alarms) | **GET** /alarms | Get a list of all alarms
[**get_alarms_id**](AlarmApi.md#get_alarms_id) | **GET** /alarms/{alarm_id} | Get a single alarm by its ID
[**get_alarms_recipients**](AlarmApi.md#get_alarms_recipients) | **GET** /alarms/{alarm_id}/recipients | Get the list of receipients of an alarm
[**get_alarms_recipients_id**](AlarmApi.md#get_alarms_recipients_id) | **GET** /alarms/{alarm_id}/recipients/{alarm_recipient_id} | Get a single recipient of an alarm by its ID
[**post_alarm**](AlarmApi.md#post_alarm) | **POST** /alarms | Create a new alarm and by that trigger signalling
[**post_alarm_recipients**](AlarmApi.md#post_alarm_recipients) | **POST** /alarms/{alarm_id}/recipients | Add a new recipient to an existing alarm
[**put_alarms_id**](AlarmApi.md#put_alarms_id) | **PUT** /alarms/{alarm_id} | Update an existing alarm
[**put_alarms_recipients_id**](AlarmApi.md#put_alarms_recipients_id) | **PUT** /alarms/{alarm_id}/recipients/{alarm_recipient_id} | Update an existing recipient of an alarm



## delete_alarms_id

> delete_alarms_id(alarm_id)
Delete an alarm by its ID

This does not cancel the alarm! Use PUT for canceling.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alarm_id** | **uuid::Uuid** | ID of the alarm | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alarms_recipients_id

> delete_alarms_recipients_id(alarm_id, alarm_recipient_id)
Delete a single alarm recipient by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alarm_id** | **uuid::Uuid** | ID of the alarm | [required] |
**alarm_recipient_id** | **uuid::Uuid** | ID of the alarm recipient | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alarms

> models::ListWrapperAlarm get_alarms()
Get a list of all alarms

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListWrapperAlarm**](ListWrapper_Alarm.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alarms_id

> models::Alarm get_alarms_id(alarm_id)
Get a single alarm by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alarm_id** | **uuid::Uuid** | ID of the alarm | [required] |

### Return type

[**models::Alarm**](Alarm.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alarms_recipients

> models::ListWrapperAlarmRecipient get_alarms_recipients(alarm_id)
Get the list of receipients of an alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alarm_id** | **uuid::Uuid** | ID of the alarm | [required] |

### Return type

[**models::ListWrapperAlarmRecipient**](ListWrapper_AlarmRecipient.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alarms_recipients_id

> models::AlarmRecipient get_alarms_recipients_id(alarm_id, alarm_recipient_id)
Get a single recipient of an alarm by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alarm_id** | **uuid::Uuid** | ID of the alarm | [required] |
**alarm_recipient_id** | **uuid::Uuid** | ID of the alarm recipient | [required] |

### Return type

[**models::AlarmRecipient**](AlarmRecipient.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_alarm

> models::Alarm post_alarm(new_alarm)
Create a new alarm and by that trigger signalling

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_alarm** | [**NewAlarm**](NewAlarm.md) | The alarm to be created | [required] |

### Return type

[**models::Alarm**](Alarm.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_alarm_recipients

> models::AlarmRecipient post_alarm_recipients(alarm_id, new_alarm_recipient)
Add a new recipient to an existing alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alarm_id** | **uuid::Uuid** | ID of the alarm | [required] |
**new_alarm_recipient** | [**NewAlarmRecipient**](NewAlarmRecipient.md) | The recipient to be added | [required] |

### Return type

[**models::AlarmRecipient**](AlarmRecipient.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_alarms_id

> models::Alarm put_alarms_id(alarm_id, new_alarm)
Update an existing alarm

Can be used for canceling the alarm by setting the field cancellation_time to a value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alarm_id** | **uuid::Uuid** | ID of the alarm | [required] |
**new_alarm** | [**NewAlarm**](NewAlarm.md) | The updated alarm | [required] |

### Return type

[**models::Alarm**](Alarm.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_alarms_recipients_id

> models::AlarmRecipient put_alarms_recipients_id(alarm_id, alarm_recipient_id, new_alarm_recipient)
Update an existing recipient of an alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alarm_id** | **uuid::Uuid** | ID of the alarm | [required] |
**alarm_recipient_id** | **uuid::Uuid** | ID of the alarm recipient | [required] |
**new_alarm_recipient** | [**NewAlarmRecipient**](NewAlarmRecipient.md) | The updated alarm recipient | [required] |

### Return type

[**models::AlarmRecipient**](AlarmRecipient.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

