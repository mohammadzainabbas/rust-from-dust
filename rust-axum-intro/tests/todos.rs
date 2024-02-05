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
async fn fetch(req: Request<Body>) -> (StatusCode, String) {
    let routers = get_routers().await;
    let res = routers
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;

    let request = Request::builder().uri(uri).body(Body::empty())?;
    let response = ServiceExt::<Request<Body>>::ready(&mut routers)
        .await?
        .call(request)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await?.to_bytes();
    assert_eq!(&body[..], format!("<h3> Hello {res}! </h3>").as_bytes());

    let app = get_routers().await;
    let response = app.oneshot(req).await.expect("failed to execute request");
    let (parts, body) = response.into_parts();
    let body_bytes = hyper::body::to_bytes(body)
        .await
        .expect("failed to read body");
    (
        parts.status,
        String::from_utf8_lossy(&body_bytes).to_string(),
    )
}

#[tokio::test]
async fn test_create_todo() -> Result<(), anyhow::Error> {
    let req = Request::builder()
        .method(http::Method::POST)
        .uri("/todo")
        .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(Body::from(json!({"text": "Test todo"}).to_string()))
        .unwrap();

    let res = fetch(req).await?;

    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await?.to_bytes();
    assert_eq!(&body[..], b"Hello, I'm groot!");

    Ok(())
}
