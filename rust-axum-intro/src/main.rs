#![allow(unused)] // for dev

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{response::Html, routing::get, Router};

#[tokio::main(worker_threads = 2)]
async fn main() {
    let router = Router::new().route("/hello", get(|| async { Html("<h3> Hello World </h3>") }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {:#?}", listener.local_addr()?);
    axum::serve(listener, router).await.unwrap();
}
