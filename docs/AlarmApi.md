# \AlarmApi

All URIs are relative to *https://alarm.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_alarms_id**](AlarmApi.md#delete_alarms_id) | **DELETE** /alarms/{id} | Delete an alarm by its ID
[**get_alarms**](AlarmApi.md#get_alarms) | **GET** /alarms | Get a list of all alarms
[**get_alarms_id**](AlarmApi.md#get_alarms_id) | **GET** /alarms/{id} | Get a single alarm by its ID
[**post_alarm**](AlarmApi.md#post_alarm) | **POST** /alarms | Create a new alarm and by that trigger signalling
[**put_alarms_id**](AlarmApi.md#put_alarms_id) | **PUT** /alarms/{id} | Update an existing alarm



## delete_alarms_id

> delete_alarms_id(id)
Delete an alarm by its ID

Delete an alarm by its ID  This does not cancel the alarm! Use PUT for canceling.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the alarm | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alarms

> crate::models::AlarmData get_alarms()
Get a list of all alarms

Get a list of all alarms

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AlarmData**](AlarmData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alarms_id

> crate::models::Alarm get_alarms_id(id)
Get a single alarm by its ID

Get a single alarm by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the alarm | [required] |

### Return type

[**crate::models::Alarm**](Alarm.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_alarm

> crate::models::Alarm post_alarm(new_alarm)
Create a new alarm and by that trigger signalling

Create a new alarm and by that trigger signalling

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_alarm** | [**NewAlarm**](NewAlarm.md) | The alarm to be created | [required] |

### Return type

[**crate::models::Alarm**](Alarm.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_alarms_id

> crate::models::Alarm put_alarms_id(id, new_alarm)
Update an existing alarm

Update an existing alarm  Can be used for canceling the alarm by setting the field cancellation_time to a value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the alarm | [required] |
**new_alarm** | [**NewAlarm**](NewAlarm.md) | The updated alarm | [required] |

### Return type

[**crate::models::Alarm**](Alarm.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

