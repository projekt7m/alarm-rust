# \AlarmApi

All URIs are relative to *https://alarm.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alarms_get**](AlarmApi.md#alarms_get) | **GET** /alarms | 
[**alarms_id_delete**](AlarmApi.md#alarms_id_delete) | **DELETE** /alarms/{id} | 
[**alarms_id_get**](AlarmApi.md#alarms_id_get) | **GET** /alarms/{id} | 
[**alarms_id_put**](AlarmApi.md#alarms_id_put) | **PUT** /alarms/{id} | 
[**alarms_post**](AlarmApi.md#alarms_post) | **POST** /alarms | 



## alarms_get

> crate::models::AlarmData alarms_get()


an alarm tracks the progress of a Telfas emergency message

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AlarmData**](AlarmData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_id_delete

> alarms_id_delete(id)


deletes the given alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | alarm id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_id_get

> crate::models::Alarm alarms_id_get(id)


Returns the given alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | alarm id | [required] |

### Return type

[**crate::models::Alarm**](Alarm.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_id_put

> crate::models::Alarm alarms_id_put(id, new_alarm)


updates the given alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | alarm id | [required] |
**new_alarm** | [**NewAlarm**](NewAlarm.md) | the updated alarm | [required] |

### Return type

[**crate::models::Alarm**](Alarm.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarms_post

> crate::models::Alarm alarms_post(new_alarm)


creates a new alarm

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_alarm** | [**NewAlarm**](NewAlarm.md) | the alarm to be created | [required] |

### Return type

[**crate::models::Alarm**](Alarm.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

