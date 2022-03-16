use std::net::SocketAddr;

use fdlimit::raise_fd_limit;
use flexi_logger::{detailed_format, Duplicate};
use hyper::Server;
use log::{error, info};
use routerify::RouterService;


use crate::router::register_router;

mod router;
mod controller;

fn init_log() {
    flexi_logger::Logger::with_str("debug")
        .log_to_file()
        .directory("log")
        .basename("rs-forum.log")
        .duplicate_to_stdout(Duplicate::All)
        .format_for_files(detailed_format)
        .format_for_stdout(detailed_format)
        .start()
        .unwrap_or_else(|e| panic!("logger initialization failed, err: {}", e));
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    info!("interrupted by CTRL+C signal");
}

#[tokio::main]
async fn main() {
    // init log
    init_log();

    // raise fd limit to max
    match raise_fd_limit() {
        Some(val) => {
            info!("raise system fd limit to {}", val);
        }
        None => {
            info!("not support to raise system fd limit")
        }
    }

    let router = register_router();
    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(service);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    info!("App is running on: {}", addr);
    if let Err(err) = graceful.await {
        error!("Server error: {}", err);
    }
}