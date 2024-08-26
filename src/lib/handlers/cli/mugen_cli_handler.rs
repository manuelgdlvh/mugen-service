use std::sync::Arc;

use crate::services::mugen_service::MugenService;

pub struct MugenCliHandler {
    service: Arc<dyn MugenService + Send + Sync>,
}

impl MugenCliHandler {
    pub fn new(service: Arc<dyn MugenService + Send + Sync>) -> Self {
        Self { service }
    }
}

impl MugenCliHandler {
    pub fn start(&self, fighter_one: &str, fighter_two: &str) {
        match self.service.start(fighter_one, fighter_two) {
            Ok(winner) => {
                log::info!("the winner is... {}", winner);
            }
            Err(err) => {
                log::error!("{}", err.backtrace());
            }
        }
    }
}

