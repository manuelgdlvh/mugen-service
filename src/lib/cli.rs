use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::sync::Arc;

use crate::handlers::cli::mugen_cli_handler::MugenCliHandler;

pub struct CLIRunner {
    handler: Arc<MugenCliHandler>,
    commands: HashMap<String, CLICommand>,
}


pub enum CLICommand {
    StartBattle
}

impl CLIRunner {
    pub fn start(&self) {
        loop {
            let mut cmd_buffer = String::new();
            log::info!("--> Please enter command: ");
            let _ = io::stdin().read_line(&mut cmd_buffer);
            self.run_command(cmd_buffer.trim());
        }
    }
    fn run_command(&self, command_type: &str) {
        if let Some(command) = self.commands.get(command_type) {
            match command {
                CLICommand::StartBattle => {
                    let mut p0_buffer = String::new();
                    let mut p1_buffer = String::new();
                    log::info!("--> Please enter player 1: ");
                    let _ = io::stdin().read_line(&mut p0_buffer);
                    log::info!("--> Please enter player 2: ");
                    let _ = io::stdin().read_line(&mut p1_buffer);
                    self.handler.start(p0_buffer.trim(), p1_buffer.trim());
                }
            }
        }
    }

    pub fn new(handler: Arc<MugenCliHandler>) -> Self {
        let mut commands = HashMap::new();
        commands.insert("start".to_string(), CLICommand::StartBattle);
        Self { handler, commands }
    }
}