// ZOMBIES (checklist when writing tests)
// Link: https://youtu.be/0_UttFDnV3k?t=3539
//
// Anything that takes a sequence (vector, array etc), test 'em with ZOM
//
// Z - zero
// O - one
// M - many
// B - boundary conditions
// I - interface
// E - exceptions
// S - simple scenarios

#![allow(unused_imports)]
use anyhow::{Ok, Result};
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
    routing::RouterIntoService,
    Router,
};
use http_body_util::BodyExt; // for `collect`
use rust_axum_intro::{get_routers, Todo};
use serde_json::{json, Value};
use tower::{Service, ServiceExt};
use uuid::Uuid; // for `call`, `oneshot`, and `ready`

async fn fetch(
    routers: &mut RouterIntoService<Body>,
    request: Request<Body>,
) -> Result<(StatusCode, String), anyhow::Error> {
    let response = ServiceExt::<Request<Body>>::ready(routers)
        .await?
        .call(request)
        .await?;
    let status = response.status();
    let body = response.into_body().collect().await?.to_bytes();
    let body = String::from_utf8_lossy(&body[..]).to_string();
    Ok((status, body))
}

#[tokio::test]
async fn test_create_todo() -> Result<(), anyhow::Error> {
    let mut routers = get_routers().await.into_service();
    let req = Request::builder()
        .method(http::Method::POST)
        .uri("/todo")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(json!({"text": "Test todo"}).to_string()))?;

    let (status, body) = fetch(&mut routers, req).await?;
    assert_eq!(status, StatusCode::CREATED);

    let todo: Todo = serde_json::from_str(&body)?;
    assert_eq!(todo.text, "Test todo");
    assert!(!todo.completed);

    Ok(())
}

#[tokio::test]
async fn test_update_todo() -> Result<(), anyhow::Error> {
    let mut routers = get_routers().await.into_service();
    // 1. First, create a todo
    let req = Request::builder()
        .method(http::Method::POST)
        .uri("/todo")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(json!({"text": "Initial todo"}).to_string()))?;

    let (status, body) = fetch(&mut routers, req).await?;
    assert_eq!(status, StatusCode::CREATED);

    let created_todo: Todo = serde_json::from_str(&body)?;
    assert_eq!(created_todo.text, "Initial todo");
    assert!(!created_todo.completed);

    // 2. Now, let's update the created todo
    let update_req = Request::builder()
        .method(http::Method::PATCH)
        .uri(format!("/todo/{}", created_todo.id))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(
            json!({"text": "Updated todo", "completed": true}).to_string(),
        ))?;

    let (status, body) = fetch(&mut routers, update_req).await?;
    assert_eq!(status, StatusCode::OK);

    let updated_todo: Todo = serde_json::from_str(&body)?;

    println!("updated_todo: {:#?}", updated_todo);

    assert_eq!(updated_todo.text, "Updated todo");
    assert!(updated_todo.completed);

    Ok(())
}

#[tokio::test]
async fn test_update_todo_no_record() -> Result<(), anyhow::Error> {
    let mut routers = get_routers().await.into_service();
    // Let's try to update a record which doesn't exist
    let random_id = Uuid::new_v4().to_string().to_owned();
    let update_req = Request::builder()
        .method(http::Method::PATCH)
        .uri(format!("/todo/{}", random_id))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(
            json!({"text": "Updated todo", "completed": true}).to_string(),
        ))?;

    let (status, _) = fetch(&mut routers, update_req).await?;
    assert_eq!(status, StatusCode::NOT_FOUND);

    Ok(())
}

async fn create_todo_list(n: usize) -> Vec<String> {
    (1..=n).map(|i| format!("Todo #{}", i)).collect()
}

#[tokio::test]
async fn test_read_todos() -> Result<(), anyhow::Error> {
    let mut routers = get_routers().await.into_service();
    // 1. Create multiple todos
    let n = 10;
    let todos = create_todo_list(n).await;
    for todo in todos {
        let req = Request::builder()
            .method(http::Method::POST)
            .uri("/todo")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json!({"text": todo}).to_string()))?;

        let (status, body) = fetch(&mut routers, req).await?;
        assert_eq!(status, StatusCode::CREATED);

        let created_todo: Todo = serde_json::from_str(&body)?;
        assert_eq!(created_todo.text, todo);
        assert!(!created_todo.completed);
    }

    // 2. Now, read the todo list
    let req = Request::builder()
        .method(http::Method::GET)
        .uri("/todo")
        .body(Body::empty())?;

    let (status, body) = fetch(&mut routers, req).await?;

    assert_eq!(status, StatusCode::OK);
    let created_todos: Vec<Todo> = serde_json::from_str(&body)?;

    assert!(!created_todos.is_empty());
    assert_eq!(created_todos.len(), n);

    Ok(())
}

#[tokio::test]
async fn test_read_todos_empty() -> Result<(), anyhow::Error> {
    let mut routers = get_routers().await.into_service();

    // Try to read the todo list (empty)
    let req = Request::builder()
        .method(http::Method::GET)
        .uri("/todo")
        .body(Body::empty())?;

    let (status, body) = fetch(&mut routers, req).await?;

    assert_eq!(status, StatusCode::NOT_FOUND);
    let todos: Vec<Todo> = serde_json::from_str(&body)?;

    assert!(todos.is_empty());
    assert_eq!(todos.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_delete_todo() -> Result<(), anyhow::Error> {
    let mut routers = get_routers().await.into_service();
    // 1. First, create a todo
    let req = Request::builder()
        .method(http::Method::POST)
        .uri("/todo")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(json!({"text": "Temp todo"}).to_string()))?;

    let (status, body) = fetch(&mut routers, req).await?;
    assert_eq!(status, StatusCode::CREATED);

    let created_todo: Todo = serde_json::from_str(&body)?;
    assert_eq!(created_todo.text, "Temp todo");
    assert!(!created_todo.completed);

    // 2. Now, let's delete the created todo
    let delete_req = Request::builder()
        .method(http::Method::DELETE)
        .uri(format!("/todo/{}", created_todo.id))
        .body(Body::empty())?;

    let (status, _) = fetch(&mut routers, delete_req).await?;
    assert_eq!(status, StatusCode::NO_CONTENT);

    // 3. Now, try to read
    let req = Request::builder()
        .method(http::Method::GET)
        .uri("/todo")
        .body(Body::empty())?;

    let (status, body) = fetch(&mut routers, req).await?;
    let todos: Vec<Todo> = serde_json::from_str(&body)?;

    if todos.is_empty() {
        assert_eq!(status, StatusCode::NOT_FOUND);
    } else {
        assert_eq!(status, StatusCode::OK);
    }

    let todo_exists = todos.iter().any(|todo| todo.id == created_todo.id);
    assert!(!todo_exists, "Deleted todo should not exist in the list.");

    Ok(())
}

#[tokio::test]
async fn test_delete_todo_no_record() -> Result<(), anyhow::Error> {
    let mut routers = get_routers().await.into_service();
    // Let's try to update a record which doesn't exist
    let random_id = Uuid::new_v4().to_string().to_owned();
    let delete_req = Request::builder()
        .method(http::Method::PATCH)
        .uri(format!("/todo/{}", random_id))
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(
            json!({"text": "Updated todo", "completed": true}).to_string(),
        ))?;

    let (status, _) = fetch(&mut routers, update_req).await?;
    assert_eq!(status, StatusCode::NOT_FOUND);

    Ok(())
}
