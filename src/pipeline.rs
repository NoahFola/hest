struct Pipeline {
    fetcher: FplFetcher,
    processor: DataProcessor,
    db: DatabaseHandler,
    seasons: Vec<String>,   // e.g. ["2022/23", "2023/24", "2024/25"]
    player_limit: u32,      // 120
}
 
impl Pipeline {
    // Entry point — setup_tables -> seed_players -> seed_history
    async fn run(&self) -> Result<()> { unimplemented!() }
}