# \AlarmTypeApi

All URIs are relative to *https://alarm.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_alarm_types**](AlarmTypeApi.md#delete_alarm_types) | **DELETE** /alarmtypes/{id} | Delete a single alarm type by its ID
[**get_alarm_types**](AlarmTypeApi.md#get_alarm_types) | **GET** /alarmtypes | Get list of all alarm types
[**get_alarm_types_id**](AlarmTypeApi.md#get_alarm_types_id) | **GET** /alarmtypes/{id} | Get a single alarm type by its ID
[**post_alarm_types**](AlarmTypeApi.md#post_alarm_types) | **POST** /alarmtypes | Create a new alarm type
[**put_alarm_types**](AlarmTypeApi.md#put_alarm_types) | **PUT** /alarmtypes/{id} | Update an existing alarm type



## delete_alarm_types

> delete_alarm_types(id)
Delete a single alarm type by its ID

Delete a single alarm type by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the alarm type | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alarm_types

> crate::models::AlarmTypeData get_alarm_types()
Get list of all alarm types

Get list of all alarm types

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AlarmTypeData**](AlarmTypeData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alarm_types_id

> crate::models::AlarmType get_alarm_types_id(id)
Get a single alarm type by its ID

Get a single alarm type by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the alarm type | [required] |

### Return type

[**crate::models::AlarmType**](AlarmType.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_alarm_types

> crate::models::AlarmType post_alarm_types(new_alarm_type)
Create a new alarm type

Create a new alarm type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_alarm_type** | [**NewAlarmType**](NewAlarmType.md) | The alarm type to be created. | [required] |

### Return type

[**crate::models::AlarmType**](AlarmType.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_alarm_types

> crate::models::AlarmType put_alarm_types(id, new_alarm_type)
Update an existing alarm type

Update an existing alarm type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the alarm type | [required] |
**new_alarm_type** | [**NewAlarmType**](NewAlarmType.md) | The updated alarm type | [required] |

### Return type

[**crate::models::AlarmType**](AlarmType.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

