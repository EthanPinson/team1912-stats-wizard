# TeamEventStatusRankRanking

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**matches_played** | Option<**i32**> | Number of matches played. | [optional]
**qual_average** | Option<**f64**> | For some years, average qualification score. Can be null. | [optional]
**sort_orders** | Option<**Vec<f64>**> | Ordered list of values used to determine the rank. See the `sort_order_info` property for the name of each value. | [optional]
**record** | Option<[**models::WltRecord**](WLT_Record.md)> |  | [optional]
**rank** | Option<**i32**> | Relative rank of this team. | [optional]
**dq** | Option<**i32**> | Number of matches the team was disqualified for. | [optional]
**team_key** | Option<**String**> | TBA team key for this rank. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


