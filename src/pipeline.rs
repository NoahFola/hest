use crate::db::DbHandler;
use crate::processor::DataProcessor;
use crate::fetcher::Fetcher;

struct Pipeline {
    fetcher: Fetcher,
    processor: DataProcessor,
    db: DbHandler,
    seasons: Vec<String>,   // e.g. ["2022/23", "2023/24", "2024/25"]
    player_limit: u32,      // 120
}
 
impl Pipeline {
    fn new(db: DbHandler, r: u32) -> Self {
        Pipeline {
            fetcher: Fetcher { http_client: reqwest::Client::new() },
            processor: DataProcessor,
            db,
            seasons: vec!["2022/23".to_string(), "2023/24".to_string(), "2024/25".to_string()],
            player_limit: r,
        }
    }

    async fn run(&self) -> Result<(),Box<dyn std::error::Error>> {
        self.db.setup_tables()?;

        let players = self.fetcher.fetch_player_ids(self.player_limit).await?;

        for player in &players {
            self.db.write_player(player.clone())?;
        }

        for player in &players {
            let history = self.fetcher.fetch_player_history(player.id).await?;
            for i in 0..history.len() {
                let prev = &history[..i];
                let current = history[i].clone();
                let processed = DataProcessor::process(&prev, current);
                self.db.write_processed_player(processed)?;
            }
        }

        Ok(())
    }
}



#[tokio::test]
async fn test_pipeline(){
    let test_pipeline_object = Pipeline::new(DbHandler::new("test".to_string()).unwrap(), 10);
    test_pipeline_object.run().await.unwrap();
}