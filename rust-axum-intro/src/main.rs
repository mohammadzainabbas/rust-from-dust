#![allow(unused)] // for dev

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{response::Html, routing::get, Router};

#[tokio::main(worker_threads = 2)]
async fn main() {
    let routes_hello =
        Router::new().route("/hello", get(|| async { Html("<h3> Hello World </h3>") }));

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    println!("Listening on {}", address);

    // axum::Serve::bind(&address).serve(routes_hello)
}
