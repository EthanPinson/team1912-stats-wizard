# EventSimple

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | TBA event key with the format yyyy[EVENT_CODE], where yyyy is the year, and EVENT_CODE is the event code of the event. | 
**name** | **String** | Official name of event on record either provided by FIRST or organizers of offseason event. | 
**event_code** | **String** | Event short code, as provided by FIRST. | 
**event_type** | **i32** | Event Type, as defined here: https://github.com/the-blue-alliance/the-blue-alliance/blob/master/consts/event_type.py#L2 | 
**district** | Option<[**models::DistrictList**](District_List.md)> |  | 
**city** | Option<**String**> | City, town, village, etc. the event is located in. | 
**state_prov** | Option<**String**> | State or Province the event is located in. | 
**country** | Option<**String**> | Country the event is located in. | 
**start_date** | [**String**](string.md) | Event start date in `yyyy-mm-dd` format. | 
**end_date** | [**String**](string.md) | Event end date in `yyyy-mm-dd` format. | 
**year** | **i32** | Year the event data is for. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


