use std::thread;
use std::time::Duration;

use ctor::{ctor, dtor};
use log::LevelFilter;
use tokio::runtime;

use lib::http_server::HttpServer;

pub const HTTP_SERVER_URL: &str = "http://localhost:3000";
pub const SERVER_URL: &str = "0.0.0.0:3000";

#[ctor]
fn init() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    thread::spawn(|| {
        runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let mut http_server = HttpServer::build(SERVER_URL).await.unwrap();
                http_server.start().await
            }).expect("http server was not initialized successfully");
    });

    log::info!("waiting to web server to be fully initialized...");
    thread::sleep(Duration::new(5, 0));
}


#[dtor]
fn destroy() {}





