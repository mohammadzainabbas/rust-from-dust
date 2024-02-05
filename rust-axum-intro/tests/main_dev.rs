use anyhow::Result;
use axum::{
    body::Body,
    http::{response, Request, StatusCode},
};
use http_body_util::BodyExt;
use rust_axum_intro::get_routers;
use tower::{Service, ServiceExt};

#[tokio::test]
async fn test_main() -> Result<()> {
    let routers = get_routers().await;

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let res = routers
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, I'm groot");

    Ok(())
}
