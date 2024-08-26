use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rand::Rng;
use wait_timeout::ChildExt;

use crate::services::mugen_stats_reader::MugenStatsReader;

const STAGES: [&str; 1] = ["dbartic"];

pub trait MugenService {
    fn start<'a>(&self, fighter_one: &'a str, fighter_two: &'a str) -> anyhow::Result<&'a str>;
}


pub struct MugenServiceImpl {
    stats_reader: Arc<dyn MugenStatsReader + Send + Sync>,
}

impl MugenService for MugenServiceImpl {
    fn start<'a>(&self, fighter_one: &'a str, fighter_two: &'a str) -> anyhow::Result<&'a str> {
        let stage_choice = STAGES[rand::thread_rng().gen_range(0..STAGES.len())];

        log::info!("starting {} vs {} battle", fighter_one, fighter_two);

        let start_command = "cd C:\\game && .\\game.exe -p1 {p1} -p2 {p2} -p1.ai 1 -p2.ai 2 -stage {stage} -rounds 1"
            .replace("{p1}", fighter_one)
            .replace("{p2}", fighter_two)
            .replace("{stage}", stage_choice);

        let _ = Command::new("cmd")
            .args(&["/c", &start_command])
            .spawn()?;

        let stats_result = self.stats_reader.start(fighter_one, fighter_two);
        let _ = Self::destroy();
        Ok(stats_result?)
    }
}

impl MugenServiceImpl {
    pub fn destroy() -> anyhow::Result<()> {
        log::info!("destroying current battle...");

        let command = "taskkill /IM game.exe";

        let mut process = Command::new("cmd")
            .args(&["/c", &command])
            .spawn()?;

        let timeout = Duration::from_secs(2);
        process.wait_timeout(timeout)?;

        if let None = process.wait_timeout(timeout)? {
            log::info!("timeout reached trying to destroy battle");
            process.kill()?;
            process.wait()?;
        }

        Ok(())
    }
    pub fn new(stats_reader: Arc<dyn MugenStatsReader + Send + Sync>) -> Self {
        Self { stats_reader }
    }
}

