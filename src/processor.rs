struct DataProcessor;
use crate::models::{RawGameweekRow, ProcessedPlayerRow};
 
impl DataProcessor {
 
    // Entry point — combines process_gameweek + engineer_features into one full DB-ready row
    fn process(prev: &[RawGameweekRow], current_gameweek_row:RawGameweekRow) -> ProcessedPlayerRow {
        let avg_fpl_last5: f32;
        let avg_goals_last5: f32;
        let avg_minutes_last5: f32;
        let mut total_fpl_last5: i32 = 0;
        let mut total_goals_last5: u32 = 0;
        let mut total_minutes_last5: u32 = 0;

        for week in prev {
            total_fpl_last5 = total_fpl_last5 + week.total_points;
            total_goals_last5 = total_goals_last5 + week.goals_scored;
            total_minutes_last5 = total_minutes_last5 + week.minutes;
        }
        avg_fpl_last5 = total_fpl_last5 as f32/5.0;
        avg_goals_last5 = total_goals_last5 as f32/5.0;
        avg_minutes_last5 = total_minutes_last5 as f32/5.0;

        let final_result: ProcessedPlayerRow = ProcessedPlayerRow {
            element: current_gameweek_row.element,
            round: current_gameweek_row.round,
            kickoff_time: current_gameweek_row.kickoff_time,
            minutes: current_gameweek_row.minutes,
            goals_scored: current_gameweek_row.goals_scored,
            assists: current_gameweek_row.assists,
            clean_sheets: current_gameweek_row.clean_sheets,
            saves: current_gameweek_row.saves,
            yellow_cards: current_gameweek_row.yellow_cards,
            red_cards: current_gameweek_row.red_cards,
            bonus: current_gameweek_row.bonus,
            was_home: current_gameweek_row.was_home,
            total_points: current_gameweek_row.total_points,
            influence: current_gameweek_row.influence,
            creativity: current_gameweek_row.creativity,
            threat: current_gameweek_row.threat,
            ict_index: current_gameweek_row.ict_index,
            expected_goals: current_gameweek_row.expected_goals,
            expected_assists: current_gameweek_row.expected_assists,
            expected_goal_involvements: current_gameweek_row.expected_goal_involvements,
            starts: current_gameweek_row.starts,
            avg_fpl_last5,
            avg_goals_last5,
            avg_minutes_last5,
        };

        return final_result;

    }
}
