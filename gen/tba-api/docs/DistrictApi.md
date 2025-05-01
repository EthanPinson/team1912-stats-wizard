# \DistrictApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_district_advancement**](DistrictApi.md#get_district_advancement) | **GET** /district/{district_key}/advancement | 
[**get_district_awards**](DistrictApi.md#get_district_awards) | **GET** /district/{district_key}/awards | 
[**get_district_events**](DistrictApi.md#get_district_events) | **GET** /district/{district_key}/events | 
[**get_district_events_keys**](DistrictApi.md#get_district_events_keys) | **GET** /district/{district_key}/events/keys | 
[**get_district_events_simple**](DistrictApi.md#get_district_events_simple) | **GET** /district/{district_key}/events/simple | 
[**get_district_history**](DistrictApi.md#get_district_history) | **GET** /district/{district_abbreviation}/history | 
[**get_district_rankings**](DistrictApi.md#get_district_rankings) | **GET** /district/{district_key}/rankings | 
[**get_district_teams**](DistrictApi.md#get_district_teams) | **GET** /district/{district_key}/teams | 
[**get_district_teams_keys**](DistrictApi.md#get_district_teams_keys) | **GET** /district/{district_key}/teams/keys | 
[**get_district_teams_simple**](DistrictApi.md#get_district_teams_simple) | **GET** /district/{district_key}/teams/simple | 
[**get_districts_by_year**](DistrictApi.md#get_districts_by_year) | **GET** /districts/{year} | 
[**get_event_district_points**](DistrictApi.md#get_event_district_points) | **GET** /event/{event_key}/district_points | 
[**get_team_districts**](DistrictApi.md#get_team_districts) | **GET** /team/{team_key}/districts | 



## get_district_advancement

> std::collections::HashMap<String, models::DistrictAdvancement> get_district_advancement(district_key, if_none_match)


Gets a list of advancement information per team in a district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**std::collections::HashMap<String, models::DistrictAdvancement>**](District_Advancement.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_awards

> Vec<models::Award> get_district_awards(district_key, if_none_match)


Gets a list of awards in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Award>**](Award.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_events

> Vec<models::Event> get_district_events(district_key, if_none_match)


Gets a list of events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Event>**](Event.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_events_keys

> Vec<String> get_district_events_keys(district_key, if_none_match)


Gets a list of event keys for events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_events_simple

> Vec<models::EventSimple> get_district_events_simple(district_key, if_none_match)


Gets a short-form list of events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::EventSimple>**](Event_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_history

> Vec<models::District> get_district_history(district_abbreviation, if_none_match)


Gets a list of District objects with the given district abbreviation. This accounts for district abbreviation changes, such as MAR to FMA.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_abbreviation** | **String** | District abbreviation, eg `ne` or `fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::District>**](District.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_rankings

> Vec<models::DistrictRanking> get_district_rankings(district_key, if_none_match)


Gets a list of team district rankings for the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::DistrictRanking>**](District_Ranking.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_teams

> Vec<models::Team> get_district_teams(district_key, if_none_match)


Gets a list of `Team` objects that competed in events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_teams_keys

> Vec<String> get_district_teams_keys(district_key, if_none_match)


Gets a list of `Team` objects that competed in events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_teams_simple

> Vec<models::TeamSimple> get_district_teams_simple(district_key, if_none_match)


Gets a short-form list of `Team` objects that competed in events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::TeamSimple>**](Team_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_districts_by_year

> Vec<models::District> get_districts_by_year(year, if_none_match)


Gets a list of districts and their corresponding district key, for the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::District>**](District.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_district_points

> models::EventDistrictPoints get_event_district_points(event_key, if_none_match)


Gets a list of district points for the Event. These are always calculated, regardless of event type, and may/may not be actually useful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**models::EventDistrictPoints**](Event_District_Points.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_districts

> Vec<models::District> get_team_districts(team_key, if_none_match)


Gets an array of districts representing each year the team was in a district. Will return an empty array if the team was never in a district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::District>**](District.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

