use crate::{basic_router, todo_router};
use axum::Router;

/// [`get_routers`] merges two routers: [`basic_router`] and [`todo_router`] into one and returns.
///
/// Note: A function that provides all app's routers makes it easy to call it from tests
/// without having to create an HTTP server.
///
/// ```
/// use
/// ```
pub async fn get_routers() -> Router {
    let basic_router = basic_router().await;
    let todo_routers = todo_router().await;
    Router::new().merge(basic_router).merge(todo_routers)
}
