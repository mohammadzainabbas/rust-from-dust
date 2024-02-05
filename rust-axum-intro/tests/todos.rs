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
use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect`
use rust_axum_intro::{get_routers, Todo};
use serde_json::{json, Value};
use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

async fn fetch(request: Request<Body>) -> Result<(StatusCode, String), anyhow::Error> {
    let routers = get_routers().await;
    let response = routers.oneshot(request).await?;
    let status = response.status();
    let body = response.into_body().collect().await?.to_bytes();
    let body = String::from_utf8_lossy(&body[..]).to_string();
    Ok((status, body))
}

#[tokio::test]
async fn test_create_todo() -> Result<(), anyhow::Error> {
    let req = Request::builder()
        .method(http::Method::POST)
        .uri("/todo")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(json!({"text": "Test todo"}).to_string()))
        .unwrap();

    let (status, body) = fetch(req).await?;
    assert_eq!(status, StatusCode::CREATED);

    let todo: Todo = serde_json::from_str(&body).unwrap();

    assert_eq!(todo.text, "Test todo");
    assert!(!todo.completed);

    Ok(())
}

#[tokio::test]
async fn test_update_todo() -> Result<(), anyhow::Error> {
    let req = Request::builder()
        .method(http::Method::POST)
        .uri("/todo")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(json!({"text": "Test todo"}).to_string()))
        .unwrap();

    let (status, body) = fetch(req).await?;
    assert_eq!(status, StatusCode::CREATED);

    let todo: Todo = serde_json::from_str(&body).unwrap();

    assert_eq!(todo.text, "Test todo");
    assert!(!todo.completed);

    Ok(())
}

#[tokio::test]
async fn test_read_todos() -> Result<(), anyhow::Error> {
    todo!()
}

#[tokio::test]
async fn test_delete_todo() -> Result<(), anyhow::Error> {
    todo!()
}
