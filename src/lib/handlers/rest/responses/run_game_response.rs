use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RunGameResponse<> {
    winner: String,
}

impl RunGameResponse {
    pub fn new(winner: String) -> Self {
        Self { winner }
    }
    pub fn winner(&self) -> &str {
        &self.winner
    }
}

