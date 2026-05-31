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