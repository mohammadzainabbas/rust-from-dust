#![allow(unused)] // for dev

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    extract::Query,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main(worker_threads = 2)]
async fn main() {
    let router = Router::new()
        .route("/", get(groot))
        .route("/hello", get(hello_world));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {:#?}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

async fn groot() -> &'static str {
    "Hello, I'm groot!"
}

async fn hello_world(Query(param): Query<HelloParams>) -> Response {
    let name = param.name.as_deref();
    Html("<h3> Hello World! </h3>").into_response()
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    pub name: Option<String>,
}
