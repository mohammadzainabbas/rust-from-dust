#![allow(unused)] // for dev

use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, RwLock},
};

use anyhow::Ok;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Error, Json, Router,
};
use clap::builder::Str;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, span, trace, warn, Level, Value};
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use uuid::Uuid;

async fn setup_tracing() {
    let log_dir = "./logs";
    let debug_file = rolling::daily(log_dir, "debug").with_min_level(Level::WARN);
    let warning_file = rolling::daily(log_dir, "warning").with_max_level(Level::WARN);
    let log_files = debug_file.and(warning_file);

    tracing_subscriber::fmt()
        .json()
        .with_writer(log_files)
        .with_ansi(true)
        .with_max_level(Level::TRACE)
        .init();

    trace!("setup_tracing() done!");
}

#[tokio::main(worker_threads = 2)]
async fn main() {
    setup_tracing().await;

    let router = Router::new()
        .route("/", get(groot))
        .route("/hello", get(say_hello))
        .route("/hello/:path", get(say_path));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("server listening on {:#?}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

async fn groot() -> Html<&'static str> {
    trace!("inside groot()");
    Html("Hello, I'm groot!")
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    pub name: Option<String>,
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

// ------------------------
// Todo CRUD
// ------------------------

type DB = Arc<RwLock<HashMap<String, Todo>>>;

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: String,
    text: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}

async fn create_todo(State(db): State<DB>, Json(input): Json<CreateTodo>) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4().to_string().to_owned(),
        text: input.text,
        completed: false,
    };

    db.write()
        .unwrap()
        .insert(todo.id.to_string(), todo.clone());

    (StatusCode::CREATED, Json(todo))
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

async fn update_todo(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = input.text {
        todo.text = text;
    }

    if let Some(completed) = input.completed {
        todo.completed = completed;
    }

    db.write().unwrap().insert(todo.id, todo.clone());

    Ok(Json(todo))
}
