use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;
use tracing::{debug, trace};

async fn groot() -> Html<&'static str> {
    trace!("inside groot()");
    Html("Hello, I'm groot!")
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tracing::instrument]
async fn say_hello(Query(param): Query<HelloParams>) -> Response {
    trace!("inside say_hello()");
    let name = param.name.as_deref().unwrap_or("World");
    debug!(target: "say", name);
    Html(format!("<h3> Hello {}! </h3>", name)).into_response()
}

#[tracing::instrument]
async fn say_path(Path(path): Path<String>) -> impl IntoResponse {
    trace!("inside say_path()");

    Html(format!("<h3> Hello {}! </h3>", path.as_str())).into_response()
}

/// Fn [`Basic Router`](basic_router) creates three basic routes, returns a [`axum::Router`](https://docs.rs/axum/latest/axum/struct.Router.html) object.
///
/// - `/` uses [`axum::routing::get`](https://docs.rs/axum/latest/axum/routing/method_routing/fn.get.html)
/// - `/hello` uses [`axum::routing::get`](https://docs.rs/axum/latest/axum/routing/method_routing/fn.get.html)
/// - `/hello/:path` uses [`axum::routing::get`](https://docs.rs/axum/latest/axum/routing/method_routing/fn.get.html)
///
pub async fn basic_router() -> Router {
    Router::new()
        .route("/", get(groot))
        .route("/hello", get(say_hello))
        .route("/hello/:path", get(say_path))
}
