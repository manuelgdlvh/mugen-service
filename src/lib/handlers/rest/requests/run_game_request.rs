use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RunGameRequest {
    #[serde(rename(deserialize = "blueFighter"))]
    #[serde(rename(serialize = "blueFighter"))]
    blue_fighter: String,
    #[serde(rename(deserialize = "redFighter"))]
    #[serde(rename(serialize = "redFighter"))]
    red_fighter: String,
}

impl RunGameRequest {
    pub fn blue_fighter(&self) -> &str {
        &self.blue_fighter
    }
    pub fn red_fighter(&self) -> &str {
        &self.red_fighter
    }
    pub fn new(blue_fighter: String, red_fighter: String) -> Self {
        Self { blue_fighter, red_fighter }
    }
}