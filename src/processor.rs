use crate::models::{RawGameweekRow, ProcessedPlayerRow};
use chrono;


pub struct DataProcessor;
impl DataProcessor {
 
    // Entry point — combines process_gameweek + engineer_features into one full DB-ready row
    pub fn process(prev: &[RawGameweekRow], current_gameweek_row: RawGameweekRow) -> ProcessedPlayerRow {
        let last3 = if prev.len() >= 3 { &prev[prev.len()-3..] } else { prev };
        let last5 = if prev.len() >= 5 { &prev[prev.len()-5..] } else { prev };
        let season = prev;

        let avg = |rows: &[RawGameweekRow], f: fn(&RawGameweekRow) -> f32| -> f32 {
            if rows.is_empty() { return 0.0; }
            rows.iter().map(f).sum::<f32>() / rows.len() as f32
        };

        let ict = |r: &RawGameweekRow| r.ict_index.parse::<f32>().unwrap_or(0.0);
        let xgi = |r: &RawGameweekRow| r.expected_goal_involvements.parse::<f32>().unwrap_or(0.0);

        let rest_days = if prev.is_empty() {
            0u32
        } else {
            let prev_date = &prev[prev.len()-1].kickoff_time;
            let curr_date = &current_gameweek_row.kickoff_time;
            let prev_parsed = chrono::DateTime::parse_from_rfc3339(prev_date).unwrap();
            let curr_parsed = chrono::DateTime::parse_from_rfc3339(curr_date).unwrap();
            (curr_parsed - prev_parsed).num_days().unsigned_abs() as u32
        };

        ProcessedPlayerRow {
            element: current_gameweek_row.element,
            round: current_gameweek_row.round,
            kickoff_time: current_gameweek_row.kickoff_time,
            total_points: current_gameweek_row.total_points,
            was_home: current_gameweek_row.was_home,
            starts: current_gameweek_row.starts,
            games_played: prev.len() as u32,
            rest_days,

            avg_fpl_last3: avg(last3, |r| r.total_points as f32),
            avg_minutes_last3: avg(last3, |r| r.minutes as f32),
            avg_goals_last3: avg(last3, |r| r.goals_scored as f32),
            avg_assists_last3: avg(last3, |r| r.assists as f32),
            avg_clean_sheets_last3: avg(last3, |r| r.clean_sheets as f32),
            avg_saves_last3: avg(last3, |r| r.saves as f32),
            avg_bonus_last3: avg(last3, |r| r.bonus as f32),
            avg_ict_last3: avg(last3, ict),
            avg_xgi_last3: avg(last3, xgi),

            avg_fpl_last5: avg(last5, |r| r.total_points as f32),
            avg_minutes_last5: avg(last5, |r| r.minutes as f32),
            avg_goals_last5: avg(last5, |r| r.goals_scored as f32),
            avg_assists_last5: avg(last5, |r| r.assists as f32),
            avg_clean_sheets_last5: avg(last5, |r| r.clean_sheets as f32),
            avg_saves_last5: avg(last5, |r| r.saves as f32),
            avg_bonus_last5: avg(last5, |r| r.bonus as f32),
            avg_ict_last5: avg(last5, ict),
            avg_xgi_last5: avg(last5, xgi),

            avg_fpl_season: avg(season, |r| r.total_points as f32),
            avg_minutes_season: avg(season, |r| r.minutes as f32),
            avg_goals_season: avg(season, |r| r.goals_scored as f32),
            avg_assists_season: avg(season, |r| r.assists as f32),
            avg_clean_sheets_season: avg(season, |r| r.clean_sheets as f32),
            avg_saves_season: avg(season, |r| r.saves as f32),
            avg_bonus_season: avg(season, |r| r.bonus as f32),
            avg_ict_season: avg(season, ict),
            avg_xgi_season: avg(season, xgi),
        }
    }
}
