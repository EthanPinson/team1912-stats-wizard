# ApiStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_season** | **i32** | Year of the current FRC season. | 
**max_season** | **i32** | Maximum FRC season year for valid queries. | 
**is_datafeed_down** | **bool** | True if the entire FMS API provided by FIRST is down. | 
**down_events** | **Vec<String>** | An array of strings containing event keys of any active events that are no longer updating. | 
**ios** | [**models::ApiStatusAppVersion**](API_Status_App_Version.md) |  | 
**android** | [**models::ApiStatusAppVersion**](API_Status_App_Version.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


