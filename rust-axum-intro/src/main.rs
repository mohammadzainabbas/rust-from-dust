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

use alloc::tracing_subscriber;

#[tokio::main(worker_threads = 2)]
async fn main() {
    tracing_subscriber::fmt::init();
    let router = Router::new()
        .route("/", get(groot))
        .route("/hello", get(say_hello))
        .route("/hello/:path", get(say_path));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {:#?}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

async fn groot() -> &'static str {
    "Hello, I'm groot!"
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
