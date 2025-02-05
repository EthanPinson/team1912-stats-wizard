# MatchSimple

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | TBA match key with the format `yyyy[EVENT_CODE]_[COMP_LEVEL]m[MATCH_NUMBER]`, where `yyyy` is the year, and `EVENT_CODE` is the event code of the event, `COMP_LEVEL` is (qm, ef, qf, sf, f), and `MATCH_NUMBER` is the match number in the competition level. A set number may append the competition level if more than one match in required per set. | 
**comp_level** | **String** | The competition level the match was played at. | 
**set_number** | **i32** | The set number in a series of matches where more than one match is required in the match series. | 
**match_number** | **i32** | The match number of the match in the competition level. | 
**alliances** | [**models::MatchSimpleAlliances**](Match_Simple_alliances.md) |  | 
**winning_alliance** | **String** | The color (red/blue) of the winning alliance. Will contain an empty string in the event of no winner, or a tie. | 
**event_key** | **String** | Event key of the event the match was played at. | 
**time** | Option<**i64**> | UNIX timestamp (seconds since 1-Jan-1970 00:00:00) of the scheduled match time, as taken from the published schedule. | 
**predicted_time** | Option<**i64**> | UNIX timestamp (seconds since 1-Jan-1970 00:00:00) of the TBA predicted match start time. | 
**actual_time** | Option<**i64**> | UNIX timestamp (seconds since 1-Jan-1970 00:00:00) of actual match start time. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


