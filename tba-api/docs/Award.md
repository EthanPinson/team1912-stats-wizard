# Award

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the award as provided by FIRST. May vary for the same award type. | 
**award_type** | **i32** | Type of award given. See https://github.com/the-blue-alliance/the-blue-alliance/blob/master/consts/award_type.py#L6 | 
**event_key** | **String** | The event_key of the event the award was won at. | 
**recipient_list** | [**Vec<models::AwardRecipient>**](Award_Recipient.md) | A list of recipients of the award at the event. May have either a team_key or an awardee, both, or neither (in the case the award wasn't awarded at the event). | 
**year** | **i32** | The year this award was won. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


