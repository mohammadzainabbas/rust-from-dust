#![allow(unused)] // for dev

use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, RwLock},
};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, patch},
    Error, Json, Router,
};
use clap::builder::Str;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, span, trace, warn, Level, Value};
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use uuid::Uuid;

pub async fn groot() -> Html<&'static str> {
    trace!("inside groot()");
    Html("Hello, I'm groot!")
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    pub name: Option<String>,
}

#[tracing::instrument]
pub async fn say_hello(Query(param): Query<HelloParams>) -> Response {
    trace!("inside say_hello()");
    let name = param.name.as_deref().unwrap_or("World");
    debug!(target: "say", name);
    Html(format!("<h3> Hello {}! </h3>", name)).into_response()
}

#[tracing::instrument]
pub async fn say_path(Path(path): Path<String>) -> impl IntoResponse {
    trace!("inside say_path()");

    Html(format!("<h3> Hello {}! </h3>", path.as_str())).into_response()
}

pub async fn basic_router() -> Router {
    Router::new()
        .route("/", get(groot))
        .route("/hello", get(say_hello))
        .route("/hello/:path", get(say_path))
}
