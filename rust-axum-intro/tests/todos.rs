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

use anyhow::Result;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect`
use rust_axum_intro::get_routers;
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
    assert_eq!(status, StatusCode::OK);

    assert_eq!(body, "Hello, I'm groot!");

    Ok(())
}
