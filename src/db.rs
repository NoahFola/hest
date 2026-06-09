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
        
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS predictions (
                player_id INTEGER NOT NULL,
                gameweek INTEGER NOT NULL,
                predicted_points REAL NOT NULL,
                predicted_at TEXT NOT NULL,
                FOREIGN KEY (player_id) REFERENCES players(id)
            )",
            rusqlite::params![],
        )?; 

        Ok(())
    }
    
    pub fn write_prediction(&self, player_id: u32, gameweek: u32, predicted_points: f64) -> Result<(), Box<dyn std::error::Error>> {
        let predicted_at = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO predictions (player_id, gameweek, predicted_points, predicted_at)
            VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![player_id, gameweek, predicted_points, predicted_at],
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

    pub fn read_latest_features(&self, player_id: u32) -> Result<ProcessedPlayerRow, Box<dyn std::error::Error>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM gameweek_stats
            WHERE element = ?1
            ORDER BY round DESC
            LIMIT 1"
        )?;

        let row = stmt.query_row(rusqlite::params![player_id], |row| {
            Ok(ProcessedPlayerRow {
                element: row.get(0)?,
                round: row.get(1)?,
                kickoff_time: row.get(2)?,
                total_points: row.get(3)?,
                was_home: row.get::<_, i32>(4)? != 0,
                starts: row.get(5)?,
                avg_fpl_last3: row.get(6)?,
                avg_minutes_last3: row.get(7)?,
                avg_goals_last3: row.get(8)?,
                avg_assists_last3: row.get(9)?,
                avg_clean_sheets_last3: row.get(10)?,
                avg_saves_last3: row.get(11)?,
                avg_bonus_last3: row.get(12)?,
                avg_ict_last3: row.get(13)?,
                avg_xgi_last3: row.get(14)?,
                avg_fpl_last5: row.get(15)?,
                avg_minutes_last5: row.get(16)?,
                avg_goals_last5: row.get(17)?,
                avg_assists_last5: row.get(18)?,
                avg_clean_sheets_last5: row.get(19)?,
                avg_saves_last5: row.get(20)?,
                avg_bonus_last5: row.get(21)?,
                avg_ict_last5: row.get(22)?,
                avg_xgi_last5: row.get(23)?,
                avg_fpl_season: row.get(24)?,
                avg_minutes_season: row.get(25)?,
                avg_goals_season: row.get(26)?,
                avg_assists_season: row.get(27)?,
                avg_clean_sheets_season: row.get(28)?,
                avg_saves_season: row.get(29)?,
                avg_bonus_season: row.get(30)?,
                avg_ict_season: row.get(31)?,
                avg_xgi_season: row.get(32)?,
                rest_days: row.get(33)?,
                games_played: row.get(34)?,
            })
        })?;

        Ok(row)
    }

    pub fn read_all_player_ids(&self) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let mut stmt = self.conn.prepare("SELECT id FROM players")?;
        let ids = stmt.query_map(rusqlite::params![], |row| row.get(0))?
            .collect::<Result<Vec<u32>, _>>()?;
        Ok(ids)
    }
}

