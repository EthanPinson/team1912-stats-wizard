# \InsightApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_insights_leaderboards_year**](InsightApi.md#get_insights_leaderboards_year) | **GET** /insights/leaderboards/{year} | 
[**get_insights_notables_year**](InsightApi.md#get_insights_notables_year) | **GET** /insights/notables/{year} | 



## get_insights_leaderboards_year

> Vec<models::LeaderboardInsight> get_insights_leaderboards_year(year, if_none_match)


Gets a list of `LeaderboardInsight` objects from a specific year. Use year=0 for overall.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::LeaderboardInsight>**](LeaderboardInsight.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_insights_notables_year

> Vec<models::NotablesInsight> get_insights_notables_year(year, if_none_match)


Gets a list of `NotablesInsight` objects from a specific year. Use year=0 for overall.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::NotablesInsight>**](NotablesInsight.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

