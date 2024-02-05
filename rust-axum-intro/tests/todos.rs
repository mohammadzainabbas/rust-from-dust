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
use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

async fn fetch(req: Request<Body>) -> (StatusCode, String) {
    let routers = get_routers().await;
    todo!()
}

#[tokio::test]
async fn test_create_todo() -> Result<(), anyhow::Error> {
    let req = Request::builder()
        .method(http::Method::POST)
        .uri("/todo")
        .header(http::header::CONTENT_TYPE, mime::)
        .body(Body::from(json!({"text": "Test todo"}).to_string()))
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await?.to_bytes();
    assert_eq!(&body[..], b"Hello, I'm groot!");

    Ok(())
}
