use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

// ------------------------
// Todo CRUD
// ------------------------

type DB = Arc<RwLock<HashMap<String, Todo>>>;

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    id: String,
    text: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
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
struct Pagination {
    offset: Option<usize>,
    limit: Option<usize>,
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

/// Fn [`todo_router`] creates four CRUD routes for `Todo`, returns a [`axum::Router`](https://docs.rs/axum/latest/axum/struct.Router.html) object.
///
/// - `/todo` with [`axum::routing::get`](https://docs.rs/axum/latest/axum/routing/method_routing/fn.get.html) method
/// - `/todo` with [`axum::routing::post`](https://docs.rs/axum/latest/axum/routing/method_routing/fn.post.html) method
/// - `/todo/:id` with [`axum::routing::patch`](https://docs.rs/axum/latest/axum/routing/method_routing/fn.patch.html) method
/// - `/todo/:id` with [`axum::routing::delete`](https://docs.rs/axum/latest/axum/routing/method_routing/fn.delete.html) method
///
pub async fn todo_router() -> Router {
    let db = DB::default();

    Router::new()
        .route("/todo", get(read_todos).post(create_todo))
        .route("/todo/:id", patch(update_todo).delete(delete_todo))
        .with_state(db)
}
