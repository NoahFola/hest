use ort::session;
use crate::db::DbHandler;

pub struct InferenceEngine {
    session: session::Session,
    db: DbHandler,
}

impl InferenceEngine {
    pub fn new(model_path: &str, db: DbHandler) -> Self {
        let session = session::builder::SessionBuilder::new().unwrap()
            .commit_from_file(model_path).unwrap();
        InferenceEngine { session, db }
    }

    pub fn predict_all(&self, gameweek: u32) -> Result<(), Box<dyn std::error::Error>> {
        let player_ids = self.db.read_all_player_ids()?;

        for player_id in player_ids {
            let row = self.db.read_latest_features(player_id)?;

            let features: Vec<f32> = vec![
                row.avg_fpl_last3 as f32,
                row.avg_minutes_last3 as f32,
                row.avg_goals_last3 as f32,
                row.avg_assists_last3 as f32,
                row.avg_clean_sheets_last3 as f32,
                row.avg_saves_last3 as f32,
                row.avg_bonus_last3 as f32,
                row.avg_ict_last3 as f32,
                row.avg_xgi_last3 as f32,
                row.avg_fpl_last5 as f32,
                row.avg_minutes_last5 as f32,
                row.avg_goals_last5 as f32,
                row.avg_assists_last5 as f32,
                row.avg_clean_sheets_last5 as f32,
                row.avg_saves_last5 as f32,
                row.avg_bonus_last5 as f32,
                row.avg_ict_last5 as f32,
                row.avg_xgi_last5 as f32,
                row.avg_fpl_season as f32,
                row.avg_minutes_season as f32,
                row.avg_goals_season as f32,
                row.avg_assists_season as f32,
                row.avg_clean_sheets_season as f32,
                row.avg_saves_season as f32,
                row.avg_bonus_season as f32,
                row.avg_ict_season as f32,
                row.avg_xgi_season as f32,
                row.rest_days as f32,
                row.games_played as f32,
            ];

            let array = ndarray::Array2::from_shape_vec((1, features.len()), features)?;
            // create input values vector for the session
            let input_values = vec![ort::value::Value::from_array(array)?];

            let outputs = self.session.run(input_values)?;
            let predicted_points = outputs[0].try_extract_tensor::<f32>()?[[0, 0]] as f64;

            self.db.write_prediction(player_id, gameweek, predicted_points)?;
        }

        Ok(())
    }
}


#[tokio::test]
async fn test_inference_engine(){
    // let test_pipeline_object = Pipeline::new(DbHandler::new("test".to_string()).unwrap(), 10);
    let test_inference_object = InferenceEngine::new("model/model.onnx",DbHandler::new("test".to_string()).unwrap());
    test_inference_object.predict_all(38)?;
}