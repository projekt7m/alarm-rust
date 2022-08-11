# \AlarmTypeApi

All URIs are relative to *https://alarm.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alarmtypes_get**](AlarmTypeApi.md#alarmtypes_get) | **GET** /alarmtypes | 
[**alarmtypes_id_delete**](AlarmTypeApi.md#alarmtypes_id_delete) | **DELETE** /alarmtypes/{id} | 
[**alarmtypes_id_get**](AlarmTypeApi.md#alarmtypes_id_get) | **GET** /alarmtypes/{id} | 
[**alarmtypes_id_put**](AlarmTypeApi.md#alarmtypes_id_put) | **PUT** /alarmtypes/{id} | 
[**alarmtypes_post**](AlarmTypeApi.md#alarmtypes_post) | **POST** /alarmtypes | 



## alarmtypes_get

> crate::models::AlarmTypeData alarmtypes_get()


an alarm type is a template for generating alarm

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AlarmTypeData**](AlarmTypeData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarmtypes_id_delete

> alarmtypes_id_delete(id)


deletes the given alarm type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | alarm type id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarmtypes_id_get

> crate::models::AlarmType alarmtypes_id_get(id)


Returns the given alarm type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | alarm type id | [required] |

### Return type

[**crate::models::AlarmType**](AlarmType.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarmtypes_id_put

> crate::models::AlarmType alarmtypes_id_put(id, new_alarm_type)


updates the given alarm type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | alarm type id | [required] |
**new_alarm_type** | [**NewAlarmType**](NewAlarmType.md) | the updated alarm type | [required] |

### Return type

[**crate::models::AlarmType**](AlarmType.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alarmtypes_post

> crate::models::AlarmType alarmtypes_post(new_alarm_type)


creates a new alarm type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_alarm_type** | [**NewAlarmType**](NewAlarmType.md) | the alarm type to be created | [required] |

### Return type

[**crate::models::AlarmType**](AlarmType.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

