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

mod hello;

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

    let db = DB::default();

    Router::new()
    .route("/", get(groot))
    .route("/hello", get(say_hello))
    .route("/hello/:path", get(say_path))

    let todo_routers = Router::new()
        .route("/todo", get(read_todos).post(create_todo))
        .route("/todo/:id", patch(update_todo).delete(delete_todo));

    let router = Router::new().merge(todo_routers).with_state(db);

    // let router = Router::new()
    //     .route("/", get(groot))
    //     .route("/hello", get(say_hello))
    //     .route("/hello/:path", get(say_path))
    //     .merge(todo_routers)
    //     // .route("/todo", get(read_todos).post(create_todo))
    //     // .route("/todo/:id", patch(update_todo).delete(delete_todo))
    //     .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("server listening on {:#?}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
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
    Path(id): Path<String>,
    State(db): State<DB>,
    Json(input): Json<UpdateTodo>,
) -> Response {
    let mut db = db.write().unwrap();

    if let Some(todo) = db.get_mut(&id) {
        if let Some(text) = input.text {
            todo.text = text;
        }
        if let Some(completed) = input.completed {
            todo.completed = completed;
        }
        (StatusCode::OK, Json(todo.clone())).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

async fn read_todos(
    pagination: Option<Query<Pagination>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let todos = db.read().unwrap();

    let Query(pagination) = pagination.unwrap_or_default();

    let todos = todos
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    let status = if todos.is_empty() {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::OK
    };

    (status, Json(todos))
}

async fn delete_todo(Path(id): Path<String>, State(db): State<DB>) -> impl IntoResponse {
    if db.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
