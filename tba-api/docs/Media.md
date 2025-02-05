# Media

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | String type of the media element. | 
**foreign_key** | **String** | The key used to identify this media on the media site. | 
**details** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | If required, a JSON dict of additional media information. | [optional]
**preferred** | Option<**bool**> | True if the media is of high quality. | [optional]
**team_keys** | **Vec<String>** | List of teams that this media belongs to. Most likely length 1. | 
**direct_url** | Option<**String**> | Direct URL to the media. | [optional]
**view_url** | Option<**String**> | The URL that leads to the full web page for the media, if one exists. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


