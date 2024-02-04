//! ### Welcome to Axum Intro Project
//!
//! Run the following via
//!
//! ```bash
//! cargo run
//! ```
//! or
//! ```bash
//! cargo watch -q -c -w src/ -x run
//! ```

use rust_axum_intro::{get_routers, setup_tracing};
use tracing::info;

#[tokio::main(worker_threads = 2)]
async fn main() {
    setup_tracing().await;

    let routers = get_routers().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("server listening on {:#?}", listener.local_addr().unwrap());
    axum::serve(listener, routers).await.unwrap();
}
