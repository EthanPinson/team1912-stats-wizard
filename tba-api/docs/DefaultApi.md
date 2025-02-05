# \DefaultApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_search_index**](DefaultApi.md#get_search_index) | **GET** /search_index | 



## get_search_index

> models::SearchIndex get_search_index(if_none_match)


Gets a large blob of data that is used on the frontend for searching. May change without notice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**models::SearchIndex**](SearchIndex.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

