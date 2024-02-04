use crate::{basic_router, todo_router};
use axum::Router;

/// *get_routers()* calls  
pub async fn get_routers() -> Router {
    let basic_router = basic_router().await;
    let todo_routers = todo_router().await;
    Router::new().merge(basic_router).merge(todo_routers)
}
