# MatchTimeseries2018

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_key** | Option<**String**> | TBA event key with the format yyyy[EVENT_CODE], where yyyy is the year, and EVENT_CODE is the event code of the event. | [optional]
**match_id** | Option<**String**> | Match ID consisting of the level, match number, and set number, eg `qm45` or `f1m1`. | [optional]
**mode** | Option<**String**> | Current mode of play, can be `pre_match`, `auto`, `telop`, or `post_match`. | [optional]
**play** | Option<**i32**> |  | [optional]
**time_remaining** | Option<**i32**> | Amount of time remaining in the match, only valid during `auto` and `teleop` modes. | [optional]
**blue_auto_quest** | Option<**i32**> | 1 if the blue alliance is credited with the AUTO QUEST, 0 if not. | [optional]
**blue_boost_count** | Option<**i32**> | Number of POWER CUBES in the BOOST section of the blue alliance VAULT. | [optional]
**blue_boost_played** | Option<**i32**> | Returns 1 if the blue alliance BOOST was played, or 0 if not played. | [optional]
**blue_current_powerup** | Option<**String**> | Name of the current blue alliance POWER UP being played, or `null`. | [optional]
**blue_face_the_boss** | Option<**i32**> | 1 if the blue alliance is credited with FACING THE BOSS, 0 if not. | [optional]
**blue_force_count** | Option<**i32**> | Number of POWER CUBES in the FORCE section of the blue alliance VAULT. | [optional]
**blue_force_played** | Option<**i32**> | Returns 1 if the blue alliance FORCE was played, or 0 if not played. | [optional]
**blue_levitate_count** | Option<**i32**> | Number of POWER CUBES in the LEVITATE section of the blue alliance VAULT. | [optional]
**blue_levitate_played** | Option<**i32**> | Returns 1 if the blue alliance LEVITATE was played, or 0 if not played. | [optional]
**blue_powerup_time_remaining** | Option<**String**> | Number of seconds remaining in the blue alliance POWER UP time, or 0 if none is active. | [optional]
**blue_scale_owned** | Option<**i32**> | 1 if the blue alliance owns the SCALE, 0 if not. | [optional]
**blue_score** | Option<**i32**> | Current score for the blue alliance. | [optional]
**blue_switch_owned** | Option<**i32**> | 1 if the blue alliance owns their SWITCH, 0 if not. | [optional]
**red_auto_quest** | Option<**i32**> | 1 if the red alliance is credited with the AUTO QUEST, 0 if not. | [optional]
**red_boost_count** | Option<**i32**> | Number of POWER CUBES in the BOOST section of the red alliance VAULT. | [optional]
**red_boost_played** | Option<**i32**> | Returns 1 if the red alliance BOOST was played, or 0 if not played. | [optional]
**red_current_powerup** | Option<**String**> | Name of the current red alliance POWER UP being played, or `null`. | [optional]
**red_face_the_boss** | Option<**i32**> | 1 if the red alliance is credited with FACING THE BOSS, 0 if not. | [optional]
**red_force_count** | Option<**i32**> | Number of POWER CUBES in the FORCE section of the red alliance VAULT. | [optional]
**red_force_played** | Option<**i32**> | Returns 1 if the red alliance FORCE was played, or 0 if not played. | [optional]
**red_levitate_count** | Option<**i32**> | Number of POWER CUBES in the LEVITATE section of the red alliance VAULT. | [optional]
**red_levitate_played** | Option<**i32**> | Returns 1 if the red alliance LEVITATE was played, or 0 if not played. | [optional]
**red_powerup_time_remaining** | Option<**String**> | Number of seconds remaining in the red alliance POWER UP time, or 0 if none is active. | [optional]
**red_scale_owned** | Option<**i32**> | 1 if the red alliance owns the SCALE, 0 if not. | [optional]
**red_score** | Option<**i32**> | Current score for the red alliance. | [optional]
**red_switch_owned** | Option<**i32**> | 1 if the red alliance owns their SWITCH, 0 if not. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


