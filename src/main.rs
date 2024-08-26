use std::sync::Arc;

use log::LevelFilter;

use lib::cli::CLIRunner;
use lib::handlers::cli::mugen_cli_handler::MugenCliHandler;
use lib::http_server::HttpServer;
use lib::services::impls::mugen_stats_win_reader::MugenStatsWinReader;
use lib::services::mugen_service::{MugenService, MugenServiceImpl};
use lib::services::mugen_stats_reader::MugenStatsReader;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    //run_cli();
    run_http_server().await;
}


fn run_cli() {
    let stats_reader: Arc<dyn MugenStatsReader + Send + Sync> = Arc::new(MugenStatsWinReader {});
    let mugen_service: Arc<dyn MugenService + Send + Sync> = Arc::new(MugenServiceImpl::new(stats_reader));
    let mugen_handler: Arc<MugenCliHandler> = Arc::new(MugenCliHandler::new(mugen_service));

    let runner = CLIRunner::new(mugen_handler);
    runner.start();
}

async fn run_http_server() {
    let mut http_server = HttpServer::build("127.0.0.1:3000").await.unwrap();
    http_server.start().await.unwrap();
}
