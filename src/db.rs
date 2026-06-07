use rusqlite;

use crate::models::{ProcessedPlayerRow, RawPlayer};

pub struct DbHandler{
    conn: rusqlite::Connection
}
impl DbHandler{
    pub fn new(r:String) -> Result<Self,Box<dyn std::error::Error>>{
        
        let temp: rusqlite::Connection = rusqlite::Connection::open(format!("data/{}.db", r))?;
        let new: DbHandler = DbHandler { conn: (temp) };
        return Ok(new);
    }

    pub  fn setup_tables(&self) -> Result<(),Box<dyn std::error::Error>> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS players (
                id INTEGER PRIMARY KEY,
                first_name TEXT NOT NULL,
                second_name TEXT NOT NULL,
                web_name TEXT NOT NULL,
                team INTEGER NOT NULL,
                element_type INTEGER NOT NULL,
                minutes INTEGER NOT NULL
            )",
            rusqlite::params![],
        )?; 

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS gameweek_stats (
                element INTEGER NOT NULL,
                round INTEGER NOT NULL,
                kickoff_time TEXT NOT NULL,
                total_points INTEGER NOT NULL,
                was_home INTEGER NOT NULL,
                starts INTEGER NOT NULL,
                avg_fpl_last3 REAL NOT NULL,
                avg_minutes_last3 REAL NOT NULL,
                avg_goals_last3 REAL NOT NULL,
                avg_assists_last3 REAL NOT NULL,
                avg_clean_sheets_last3 REAL NOT NULL,
                avg_saves_last3 REAL NOT NULL,
                avg_bonus_last3 REAL NOT NULL,
                avg_ict_last3 REAL NOT NULL,
                avg_xgi_last3 REAL NOT NULL,
                avg_fpl_last5 REAL NOT NULL,
                avg_minutes_last5 REAL NOT NULL,
                avg_goals_last5 REAL NOT NULL,
                avg_assists_last5 REAL NOT NULL,
                avg_clean_sheets_last5 REAL NOT NULL,
                avg_saves_last5 REAL NOT NULL,
                avg_bonus_last5 REAL NOT NULL,
                avg_ict_last5 REAL NOT NULL,
                avg_xgi_last5 REAL NOT NULL,
                avg_fpl_season REAL NOT NULL,
                avg_minutes_season REAL NOT NULL,
                avg_goals_season REAL NOT NULL,
                avg_assists_season REAL NOT NULL,
                avg_clean_sheets_season REAL NOT NULL,
                avg_saves_season REAL NOT NULL,
                avg_bonus_season REAL NOT NULL,
                avg_ict_season REAL NOT NULL,
                avg_xgi_season REAL NOT NULL,
                rest_days INTEGER NOT NULL,
                games_played INTEGER NOT NULL,
                FOREIGN KEY (element) REFERENCES players(id)
            )",
            rusqlite::params![],
        )?;

        Ok(())
    }

    pub fn write_player(&self, r: RawPlayer) -> Result<(),Box<dyn std::error::Error>> {
        self.conn.execute(
            "INSERT INTO players (id, first_name, second_name, web_name, team, element_type, minutes)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![r.id, r.first_name, r.second_name, r.web_name, r.team, r.element_type, r.minutes],
        )?;
        Ok(())
    }

    pub fn write_processed_player(&self, p: ProcessedPlayerRow) -> Result<(),Box<dyn std::error::Error>> {
        self.conn.execute(
            "INSERT INTO gameweek_stats (
                element, round, kickoff_time, total_points, was_home, starts,
                avg_fpl_last3, avg_minutes_last3, avg_goals_last3, avg_assists_last3,
                avg_clean_sheets_last3, avg_saves_last3, avg_bonus_last3, avg_ict_last3, avg_xgi_last3,
                avg_fpl_last5, avg_minutes_last5, avg_goals_last5, avg_assists_last5,
                avg_clean_sheets_last5, avg_saves_last5, avg_bonus_last5, avg_ict_last5, avg_xgi_last5,
                avg_fpl_season, avg_minutes_season, avg_goals_season, avg_assists_season,
                avg_clean_sheets_season, avg_saves_season, avg_bonus_season, avg_ict_season, avg_xgi_season,
                rest_days, games_played
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30,
                ?31, ?32, ?33, ?34, ?35
            )",
            rusqlite::params![
                p.element, p.round, p.kickoff_time, p.total_points, p.was_home as i32, p.starts,
                p.avg_fpl_last3 as f64, p.avg_minutes_last3 as f64, p.avg_goals_last3 as f64, p.avg_assists_last3 as f64,
                p.avg_clean_sheets_last3 as f64, p.avg_saves_last3 as f64, p.avg_bonus_last3 as f64, p.avg_ict_last3 as f64, p.avg_xgi_last3 as f64,
                p.avg_fpl_last5 as f64, p.avg_minutes_last5 as f64, p.avg_goals_last5 as f64, p.avg_assists_last5 as f64,
                p.avg_clean_sheets_last5 as f64, p.avg_saves_last5 as f64, p.avg_bonus_last5 as f64, p.avg_ict_last5 as f64, p.avg_xgi_last5 as f64,
                p.avg_fpl_season as f64, p.avg_minutes_season as f64, p.avg_goals_season as f64, p.avg_assists_season as f64,
                p.avg_clean_sheets_season as f64, p.avg_saves_season as f64, p.avg_bonus_season as f64, p.avg_ict_season as f64, p.avg_xgi_season as f64,
                p.rest_days, p.games_played
            ],
        )?;
        Ok(())
    }

}

