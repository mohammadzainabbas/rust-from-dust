use axum::Router;
use rust_axum_intro::{basic_router, setup_tracing, todo_router};
use tracing::info;

#[tokio::main(worker_threads = 2)]
async fn main() {
    setup_tracing().await;

    let basic_router = basic_router().await;
    let todo_routers = todo_router().await;
    let routers = Router::new().merge(basic_router).merge(todo_routers);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("server listening on {:#?}", listener.local_addr().unwrap());
    axum::serve(listener, routers).await.unwrap();
}
