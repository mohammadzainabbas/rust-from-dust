// #![allow(unused)] // for dev

use axum::{Router, routing::get, response::Html};

#[tokio::main(worker_threads = 2)]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(
        || async {
            Html("<h3> Hello World </h3>")
        }
    ))
}
