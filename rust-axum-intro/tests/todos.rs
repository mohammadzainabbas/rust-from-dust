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
    http::{Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect`
use rust_axum_intro::get_routers;
use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

#[tokio::test]
async fn test_create_todo() -> Result<(), anyhow::Error> {
    let routers = get_routers().await;

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let res = routers
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;

    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await?.to_bytes();
    assert_eq!(&body[..], b"Hello, I'm groot!");

    Ok(())
}
