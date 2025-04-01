# \RegionalAdvancementApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_regional_advancement**](RegionalAdvancementApi.md#get_regional_advancement) | **GET** /regional_advancement/{year} | 
[**get_regional_rankings**](RegionalAdvancementApi.md#get_regional_rankings) | **GET** /regional_advancement/{year}/rankings | 



## get_regional_advancement

> std::collections::HashMap<String, models::RegionalAdvancement> get_regional_advancement(year, if_none_match)


Gets information about per-team advancement to the FIRST Championship.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**std::collections::HashMap<String, models::RegionalAdvancement>**](Regional_Advancement.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_regional_rankings

> Vec<models::RegionalRanking> get_regional_rankings(year, if_none_match)


Gets the team rankings in the regional pool for a specific year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::RegionalRanking>**](Regional_Ranking.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

