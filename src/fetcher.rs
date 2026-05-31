use reqwest::{self, Client, Error};
use serde::Deserialize;
//use DatabaseConnection;
use crate::models::RawPlayer;


#[derive(Deserialize)]
struct PlayerIdResponse {
    elements : Vec<RawPlayer>
}

#[derive(Debug)]
struct Fetcher{
    http_client: reqwest::Client,
    //db: DatabaseConnection
}

impl Fetcher {
    pub async fn fetch_player_ids(&self, limit:u32) -> Result<Vec<RawPlayer> , Box<dyn std::error::Error>> {
        let result: reqwest::Response= self.http_client.get("https://fantasy.premierleague.com/api/bootstrap-static/").send().await?;
        let mut final_result : PlayerIdResponse = result.json().await?;
        final_result.elements.sort_by(|a: &RawPlayer,b: &RawPlayer|{    
            b.minutes.cmp(&a.minutes)
        });
        return Ok(final_result.elements.into_iter().take(limit as usize).collect())

   }




//    async fn fetch_player_history(&self, player_id: u32) -> Result<()>{

//    }    
}


#[tokio::test]
async fn test_fetch_player_ids(){
    let test_fetcher: Fetcher = Fetcher{http_client: Client::new()};
    let t = test_fetcher.fetch_player_ids( 10).await.unwrap();
    println!("{:#?}",t)
}