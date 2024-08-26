use std::sync::Arc;

use axum::Router;
use axum::routing::post;
use tokio::net::TcpListener;

use crate::handlers::rest::mugen_rest_handler;
use crate::services::mugen_service::{MugenService, MugenServiceImpl};
use crate::services::mugen_stats_reader::MugenStatsReader;

pub struct HttpServer {
    listener: Option<TcpListener>,
    app: Option<Router>,
    started: bool,
}


impl HttpServer {
    pub async fn build(listener: &str) -> anyhow::Result<Self> {

        // Dependencies
        let stats_reader: Arc<dyn MugenStatsReader + Send + Sync> = Arc::new(crate::services::impls::mugen_stats_win_reader::MugenStatsWinReader {});
        let mugen_service: Arc<dyn MugenService + Send + Sync> = Arc::new(MugenServiceImpl::new(stats_reader));

        // Routes
        let app = Router::new()
            .route("/run", post(mugen_rest_handler::start)).with_state(mugen_service);
        let listener = TcpListener::bind(listener).await?;

        Ok(Self {
            listener: Some(listener),
            app: Some(app),
            started: false,
        })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        log::info!("starting web server...");
        if self.started {
            return Ok(());
        }

        let listener = self.listener.take().unwrap();
        let app = self.app.take().unwrap();
        self.started = true;

        axum::serve(listener, app).await?;
        Ok(())
    }
}

pub async fn run() {}