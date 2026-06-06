use core::f32;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RawPlayer {
    pub id: u32,
    pub first_name: String, 
    pub second_name: String,
    pub web_name: String, 
    pub team: u32,
    pub element_type: u32,
    pub minutes:u32
}

#[derive(Deserialize, Debug)]
pub struct RawGameweekRow {
    pub element: u32,
    pub round: u32,
    pub kickoff_time: String,
    pub minutes: u32,
    pub goals_scored: u32,
    pub assists: u32,
    pub clean_sheets: u32,
    pub saves: u32,
    pub yellow_cards: u32,
    pub red_cards: u32,
    pub bonus: u32,
    pub was_home: bool,
    pub total_points: i32,
    pub influence: String,
    pub creativity: String,
    pub threat: String,
    pub ict_index: String,
    pub expected_goals: String,
    pub expected_assists: String,
    pub expected_goal_involvements: String,
    pub starts: u32,
}


#[derive(Deserialize, Debug)]
pub struct ProcessedPlayerRow {
    pub element: u32,
    pub round: u32,
    pub kickoff_time: String,
    pub minutes: u32,
    pub goals_scored: u32,
    pub assists: u32,
    pub clean_sheets: u32,
    pub saves: u32,
    pub yellow_cards: u32,
    pub red_cards: u32,
    pub bonus: u32,
    pub was_home: bool,
    pub total_points: i32,
    pub influence: String,
    pub creativity: String,
    pub threat: String,
    pub ict_index: String,
    pub expected_goals: String,
    pub expected_assists: String,
    pub expected_goal_involvements: String,
    pub starts: u32,
    pub avg_fpl_last5: f32,
    pub avg_goals_last5: f32,
    pub avg_minutes_last5: f32,
}