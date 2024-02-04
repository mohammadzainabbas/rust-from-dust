// #![allow(unused)] // for dev

use axum::Router;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, span, trace, warn, Level, Value};
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use uuid::Uuid;

mod hello;
mod todos;

async fn setup_tracing() {
    let log_dir = "./logs";
    let debug_file = rolling::daily(log_dir, "debug").with_min_level(Level::WARN);
    let warning_file = rolling::daily(log_dir, "warning").with_max_level(Level::WARN);
    let log_files = debug_file.and(warning_file);

    tracing_subscriber::fmt()
        .json()
        .with_writer(log_files)
        .with_ansi(true)
        .with_max_level(Level::TRACE)
        .init();

    trace!("setup_tracing() done!");
}

#[tokio::main(worker_threads = 2)]
async fn main() {
    setup_tracing().await;

    let basic_router = hello::basic_router().await;
    let todo_routers = todos::todo_router().await;
    let routers = Router::new().merge(basic_router).merge(todo_routers);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("server listening on {:#?}", listener.local_addr().unwrap());
    axum::serve(listener, routers).await.unwrap();
}
