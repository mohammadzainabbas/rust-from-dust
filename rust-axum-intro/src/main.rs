#![allow(unused)] // for dev

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

#[tokio::main(worker_threads = 2)]
async fn main() {
    let router = Router::new().route("/hello", get(hello_world));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {:#?}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

async fn hello_world() -> Response {
    Html("<h3> Hello World! </h3>").into_response()
}

async fn root() -> &'static str {
    "Hello root!"
}
