struct Pipeline {
    fetcher: FplFetcher,
    processor: DataProcessor,
    db: DatabaseHandler,
    seasons: Vec<String>,   // e.g. ["2022/23", "2023/24", "2024/25"]
    player_limit: u32,      // 120
}
 
impl Pipeline {
    // Create all tables in the DB if they don't exist
    fn setup_tables(&self) -> Result<()> { unimplemented!() }
 
    // Fetch and store the 120 players of interest from bootstrap-static
    async fn seed_players(&self) -> Result<()> { unimplemented!() }
 
    // Fetch and store 3 seasons of historical data for all tracked players
    async fn seed_history(&self) -> Result<()> { unimplemented!() }
 
    // Entry point — setup_tables -> seed_players -> seed_history
    async fn run(&self) -> Result<()> { unimplemented!() }
}