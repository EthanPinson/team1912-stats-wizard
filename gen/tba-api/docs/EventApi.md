# \EventApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_district_awards**](EventApi.md#get_district_awards) | **GET** /district/{district_key}/awards | 
[**get_district_events**](EventApi.md#get_district_events) | **GET** /district/{district_key}/events | 
[**get_district_events_keys**](EventApi.md#get_district_events_keys) | **GET** /district/{district_key}/events/keys | 
[**get_district_events_simple**](EventApi.md#get_district_events_simple) | **GET** /district/{district_key}/events/simple | 
[**get_event**](EventApi.md#get_event) | **GET** /event/{event_key} | 
[**get_event_advancement_points**](EventApi.md#get_event_advancement_points) | **GET** /event/{event_key}/advancement_points | 
[**get_event_alliances**](EventApi.md#get_event_alliances) | **GET** /event/{event_key}/alliances | 
[**get_event_awards**](EventApi.md#get_event_awards) | **GET** /event/{event_key}/awards | 
[**get_event_coprs**](EventApi.md#get_event_coprs) | **GET** /event/{event_key}/coprs | 
[**get_event_district_points**](EventApi.md#get_event_district_points) | **GET** /event/{event_key}/district_points | 
[**get_event_insights**](EventApi.md#get_event_insights) | **GET** /event/{event_key}/insights | 
[**get_event_match_timeseries**](EventApi.md#get_event_match_timeseries) | **GET** /event/{event_key}/matches/timeseries | 
[**get_event_matches**](EventApi.md#get_event_matches) | **GET** /event/{event_key}/matches | 
[**get_event_matches_keys**](EventApi.md#get_event_matches_keys) | **GET** /event/{event_key}/matches/keys | 
[**get_event_matches_simple**](EventApi.md#get_event_matches_simple) | **GET** /event/{event_key}/matches/simple | 
[**get_event_oprs**](EventApi.md#get_event_oprs) | **GET** /event/{event_key}/oprs | 
[**get_event_predictions**](EventApi.md#get_event_predictions) | **GET** /event/{event_key}/predictions | 
[**get_event_rankings**](EventApi.md#get_event_rankings) | **GET** /event/{event_key}/rankings | 
[**get_event_simple**](EventApi.md#get_event_simple) | **GET** /event/{event_key}/simple | 
[**get_event_team_media**](EventApi.md#get_event_team_media) | **GET** /event/{event_key}/team_media | 
[**get_event_teams**](EventApi.md#get_event_teams) | **GET** /event/{event_key}/teams | 
[**get_event_teams_keys**](EventApi.md#get_event_teams_keys) | **GET** /event/{event_key}/teams/keys | 
[**get_event_teams_simple**](EventApi.md#get_event_teams_simple) | **GET** /event/{event_key}/teams/simple | 
[**get_event_teams_statuses**](EventApi.md#get_event_teams_statuses) | **GET** /event/{event_key}/teams/statuses | 
[**get_events_by_year**](EventApi.md#get_events_by_year) | **GET** /events/{year} | 
[**get_events_by_year_keys**](EventApi.md#get_events_by_year_keys) | **GET** /events/{year}/keys | 
[**get_events_by_year_simple**](EventApi.md#get_events_by_year_simple) | **GET** /events/{year}/simple | 
[**get_regional_champs_pool_points**](EventApi.md#get_regional_champs_pool_points) | **GET** /event/{event_key}/regional_champs_pool_points | 
[**get_team_event_awards**](EventApi.md#get_team_event_awards) | **GET** /team/{team_key}/event/{event_key}/awards | 
[**get_team_event_matches**](EventApi.md#get_team_event_matches) | **GET** /team/{team_key}/event/{event_key}/matches | 
[**get_team_event_matches_keys**](EventApi.md#get_team_event_matches_keys) | **GET** /team/{team_key}/event/{event_key}/matches/keys | 
[**get_team_event_matches_simple**](EventApi.md#get_team_event_matches_simple) | **GET** /team/{team_key}/event/{event_key}/matches/simple | 
[**get_team_event_status**](EventApi.md#get_team_event_status) | **GET** /team/{team_key}/event/{event_key}/status | 
[**get_team_events**](EventApi.md#get_team_events) | **GET** /team/{team_key}/events | 
[**get_team_events_by_year**](EventApi.md#get_team_events_by_year) | **GET** /team/{team_key}/events/{year} | 
[**get_team_events_by_year_keys**](EventApi.md#get_team_events_by_year_keys) | **GET** /team/{team_key}/events/{year}/keys | 
[**get_team_events_by_year_simple**](EventApi.md#get_team_events_by_year_simple) | **GET** /team/{team_key}/events/{year}/simple | 
[**get_team_events_keys**](EventApi.md#get_team_events_keys) | **GET** /team/{team_key}/events/keys | 
[**get_team_events_simple**](EventApi.md#get_team_events_simple) | **GET** /team/{team_key}/events/simple | 
[**get_team_events_statuses_by_year**](EventApi.md#get_team_events_statuses_by_year) | **GET** /team/{team_key}/events/{year}/statuses | 



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


## get_event

> models::Event get_event(event_key, if_none_match)


Gets an Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**models::Event**](Event.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_advancement_points

> models::EventDistrictPoints get_event_advancement_points(event_key, if_none_match)


Depending on the type of event (district/regional), this will return either district points or regional CMP points

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


## get_event_alliances

> Vec<models::EliminationAlliance> get_event_alliances(event_key, if_none_match)


Gets a list of Elimination Alliances for the given Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::EliminationAlliance>**](Elimination_Alliance.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_awards

> Vec<models::Award> get_event_awards(event_key, if_none_match)


Gets a list of awards from the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Award>**](Award.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_coprs

> std::collections::HashMap<String, std::collections::HashMap<String, f64>> get_event_coprs(event_key, if_none_match)


Gets a set of Event Component OPRs for the given Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**std::collections::HashMap<String, std::collections::HashMap<String, f64>>**](std::collections::HashMap.md)

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


## get_event_insights

> models::EventInsights get_event_insights(event_key, if_none_match)


Gets a set of Event-specific insights for the given Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**models::EventInsights**](Event_Insights.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_match_timeseries

> Vec<String> get_event_match_timeseries(event_key, if_none_match)


Gets an array of Match Keys for the given event key that have timeseries data. Returns an empty array if no matches have timeseries data. *WARNING:* This is *not* official data, and is subject to a significant possibility of error, or missing data. Do not rely on this data for any purpose. In fact, pretend we made it up. *WARNING:* This endpoint and corresponding data models are under *active development* and may change at any time, including in breaking ways.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_matches

> Vec<models::Match> get_event_matches(event_key, if_none_match)


Gets a list of matches for the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Match>**](Match.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_matches_keys

> Vec<String> get_event_matches_keys(event_key, if_none_match)


Gets a list of match keys for the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_matches_simple

> Vec<models::MatchSimple> get_event_matches_simple(event_key, if_none_match)


Gets a short-form list of matches for the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::MatchSimple>**](Match_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_oprs

> models::EventOprs get_event_oprs(event_key, if_none_match)


Gets a set of Event OPRs (including OPR, DPR, and CCWM) for the given Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**models::EventOprs**](Event_OPRs.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_predictions

> serde_json::Value get_event_predictions(event_key, if_none_match)


Gets information on TBA-generated predictions for the given Event. Contains year-specific information. *WARNING* This endpoint is currently under development and may change at any time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_rankings

> models::EventRanking get_event_rankings(event_key, if_none_match)


Gets a list of team rankings for the Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**models::EventRanking**](Event_Ranking.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_simple

> models::EventSimple get_event_simple(event_key, if_none_match)


Gets a short-form Event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**models::EventSimple**](Event_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_team_media

> Vec<models::Media> get_event_team_media(event_key, if_none_match)


Gets a list of media objects that correspond to teams at this event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Media>**](Media.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_teams

> Vec<models::Team> get_event_teams(event_key, if_none_match)


Gets a list of `Team` objects that competed in the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_teams_keys

> Vec<String> get_event_teams_keys(event_key, if_none_match)


Gets a list of `Team` keys that competed in the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_teams_simple

> Vec<models::TeamSimple> get_event_teams_simple(event_key, if_none_match)


Gets a short-form list of `Team` objects that competed in the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::TeamSimple>**](Team_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_teams_statuses

> std::collections::HashMap<String, models::TeamEventStatus> get_event_teams_statuses(event_key, if_none_match)


Gets a key-value list of the event statuses for teams competing at the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**std::collections::HashMap<String, models::TeamEventStatus>**](Team_Event_Status.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_by_year

> Vec<models::Event> get_events_by_year(year, if_none_match)


Gets a list of events in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Event>**](Event.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_by_year_keys

> Vec<String> get_events_by_year_keys(year, if_none_match)


Gets a list of event keys in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_by_year_simple

> Vec<models::EventSimple> get_events_by_year_simple(year, if_none_match)


Gets a short-form list of events in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::EventSimple>**](Event_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_regional_champs_pool_points

> models::EventDistrictPoints get_regional_champs_pool_points(event_key, if_none_match)


For 2025+ Regional events, this will return points towards the Championship qualification pool.

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


## get_team_event_awards

> Vec<models::Award> get_team_event_awards(team_key, event_key, if_none_match)


Gets a list of awards the given team won at the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Award>**](Award.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_event_matches

> Vec<models::Match> get_team_event_matches(team_key, event_key, if_none_match)


Gets a list of matches for the given team and event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Match>**](Match.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_event_matches_keys

> Vec<String> get_team_event_matches_keys(team_key, event_key, if_none_match)


Gets a list of match keys for matches for the given team and event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_event_matches_simple

> Vec<models::Match> get_team_event_matches_simple(team_key, event_key, if_none_match)


Gets a short-form list of matches for the given team and event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Match>**](Match.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_event_status

> models::TeamEventStatus get_team_event_status(team_key, event_key, if_none_match)


Gets the competition rank and status of the team at the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**models::TeamEventStatus**](Team_Event_Status.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events

> Vec<models::Event> get_team_events(team_key, if_none_match)


Gets a list of all events this team has competed at.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Event>**](Event.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events_by_year

> Vec<models::Event> get_team_events_by_year(team_key, year, if_none_match)


Gets a list of events this team has competed at in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Event>**](Event.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events_by_year_keys

> Vec<String> get_team_events_by_year_keys(team_key, year, if_none_match)


Gets a list of the event keys for events this team has competed at in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events_by_year_simple

> Vec<models::EventSimple> get_team_events_by_year_simple(team_key, year, if_none_match)


Gets a short-form list of events this team has competed at in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::EventSimple>**](Event_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events_keys

> Vec<String> get_team_events_keys(team_key, if_none_match)


Gets a list of the event keys for all events this team has competed at.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events_simple

> Vec<models::EventSimple> get_team_events_simple(team_key, if_none_match)


Gets a short-form list of all events this team has competed at.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::EventSimple>**](Event_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events_statuses_by_year

> std::collections::HashMap<String, models::TeamEventStatus> get_team_events_statuses_by_year(team_key, year, if_none_match)


Gets a key-value list of the event statuses for events this team has competed at in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**std::collections::HashMap<String, models::TeamEventStatus>**](Team_Event_Status.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

