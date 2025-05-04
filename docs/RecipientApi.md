# \RecipientApi

All URIs are relative to *https://alarm.api.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_recipients_recipient_id**](RecipientApi.md#delete_recipients_recipient_id) | **DELETE** /recipients/{recipient_id} | Delete a single recipient by its ID
[**get_recipients**](RecipientApi.md#get_recipients) | **GET** /recipients | Get the list of all recipients
[**get_recipients_recipient_id**](RecipientApi.md#get_recipients_recipient_id) | **GET** /recipients/{recipient_id} | Get a single recipient by its ID
[**post_recipients**](RecipientApi.md#post_recipients) | **POST** /recipients | Create a new recipient
[**put_recipients_recipient_id**](RecipientApi.md#put_recipients_recipient_id) | **PUT** /recipients/{recipient_id} | Update an existing recipient



## delete_recipients_recipient_id

> delete_recipients_recipient_id(recipient_id)
Delete a single recipient by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipient_id** | **uuid::Uuid** | ID of the recipient | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recipients

> models::ListWrapperRecipient get_recipients()
Get the list of all recipients

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListWrapperRecipient**](ListWrapper_Recipient.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recipients_recipient_id

> models::Recipient get_recipients_recipient_id(recipient_id)
Get a single recipient by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipient_id** | **uuid::Uuid** | ID of the recipient | [required] |

### Return type

[**models::Recipient**](Recipient.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recipients

> models::Recipient post_recipients(new_recipient)
Create a new recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_recipient** | [**NewRecipient**](NewRecipient.md) | The receipient to be created | [required] |

### Return type

[**models::Recipient**](Recipient.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_recipients_recipient_id

> models::Recipient put_recipients_recipient_id(recipient_id, new_recipient)
Update an existing recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipient_id** | **uuid::Uuid** | ID of the recipient | [required] |
**new_recipient** | [**NewRecipient**](NewRecipient.md) | The updated recipient | [required] |

### Return type

[**models::Recipient**](Recipient.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

