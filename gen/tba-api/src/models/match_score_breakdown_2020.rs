/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).
 *
 * The version of the OpenAPI document: 3.9.11
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// MatchScoreBreakdown2020 : See the 2020 FMS API documentation for a description of each value. https://frcevents2.docs.apiary.io/#/reference/match-results/score-details
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchScoreBreakdown2020 {
    #[serde(rename = "blue")]
    pub blue: Box<models::MatchScoreBreakdown2020Alliance>,
    #[serde(rename = "red")]
    pub red: Box<models::MatchScoreBreakdown2020Alliance>,
}

impl MatchScoreBreakdown2020 {
    /// See the 2020 FMS API documentation for a description of each value. https://frcevents2.docs.apiary.io/#/reference/match-results/score-details
    pub fn new(blue: models::MatchScoreBreakdown2020Alliance, red: models::MatchScoreBreakdown2020Alliance) -> MatchScoreBreakdown2020 {
        MatchScoreBreakdown2020 {
            blue: Box::new(blue),
            red: Box::new(red),
        }
    }
}

