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

// ------------------------
// Todo CRUD
// ------------------------

pub type DB = Arc<RwLock<HashMap<String, Todo>>>;

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub text: String,
}

pub async fn create_todo(State(db): State<DB>, Json(input): Json<CreateTodo>) -> impl IntoResponse {
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
pub struct UpdateTodo {
    pub text: Option<String>,
    pub completed: Option<bool>,
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
