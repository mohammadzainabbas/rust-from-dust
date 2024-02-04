#![allow(unused)] // for dev

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use clap::builder::Str;
use serde::Deserialize;
use tracing::{debug, error, info, span, trace, warn, Level};
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

async fn setup_tracing() {
    let log_dir = "./logs";
    let debug_file = rolling::daily(log_dir, "debug");
    let warning_file = rolling::daily(log_dir, "warning").with_max_level(Level::WARN);
    let log_files = debug_file.and(warning_file);

    tracing_subscriber::fmt()
        .json()
        .with_writer(log_files)
        .with_ansi(true)
        .init();
}

#[tokio::main(worker_threads = 2)]
async fn main() {
    setup_tracing().await;

    trace!("setup_tracing() done!");

    let router = Router::new()
        .route("/", get(groot))
        .route("/hello", get(say_hello))
        .route("/hello/:path", get(say_path));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on {:#?}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

async fn groot() -> Html<&'static str> {
    // debug!("")
    Html("Hello, I'm groot!")
}

async fn say_hello(Query(param): Query<HelloParams>) -> Response {
    let name = param.name.as_deref().unwrap_or("World");
    Html(format!("<h3> Hello {}! </h3>", name)).into_response()
}

async fn say_path(Path(path): Path<String>) -> impl IntoResponse {
    Html(format!("<h3> Hello {}! </h3>", path.as_str())).into_response()
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    pub name: Option<String>,
}
