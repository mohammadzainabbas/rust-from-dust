use crate::{basic_router, setup_tracing, todo_router};
use axum::Router;

pub async fn routers() -> Router {
    let basic_router = basic_router().await;
    let todo_routers = todo_router().await;
    Router::new().merge(basic_router).merge(todo_routers)
}
