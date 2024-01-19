#![allow(unused)] // for dev

use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main(worker_threads = 2)]
async fn main() {
    let routes_hello =
        Router::new().route("/hello", get(|| async { Html("<h3> Hello World </h3>") }));

    let socket = SocketAddr::new(ip, port)
}
