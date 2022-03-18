#[macro_use]
extern crate lazy_static;

use std::net::SocketAddr;

use fdlimit::raise_fd_limit;
use flexi_logger::{detailed_format, Duplicate};
use hyper::Server;
use log::info;
use routerify::RouterService;
use tokio::signal;

use crate::router::register_router;
use crate::utils::g::init_mysql_rbatis_session;

mod router;
mod controller;
mod utils;
mod model;

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
    #[cfg(unix)]
        let ctrl_c = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install Ctrl+C handler")
            .recv()
            .await;
        info!("terminated by SIGINT");
    };

    #[cfg(not(unix))]
        let ctrl_c = async {
        signal::windows::ctrl_c().unwrap().recv()
            .await
            .expect("failed to install Ctrl+C handler");
        info!("terminated by Ctrl+C");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
        info!("terminated by SIGTERM");
    };

    #[cfg(not(unix))]
        let terminate = async {
        signal::windows::ctrl_break().unwrap().recv()
            .await
            .expect("failed to install Ctrl+Break handler");
        info!("terminated by Ctrl+Break");
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    // init log
    init_log();

    // init db session
    init_mysql_rbatis_session().await;

    // raise fd limit to max
    match raise_fd_limit() {
        Some(val) => {
            info!("raise system fd limit to {}", val);
        }
        None => {
            info!("not support to raise system fd limit")
        }
    }

    let listen_addr_str = "0.0.0.0:3000";
    let listen_addr: SocketAddr = listen_addr_str.parse().unwrap();

    let router = register_router();
    let service = RouterService::new(router).unwrap();

    info!("App is running on: {}", listen_addr_str);
    Server::bind(&listen_addr).serve(service)
        .with_graceful_shutdown(shutdown_signal()).await.unwrap();
}