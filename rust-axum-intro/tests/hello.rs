#![allow(unused_imports)]

use anyhow::Result;
use axum::{
    body::Body,
    extract::connect_info::MockConnectInfo,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect`
use rust_axum_intro::get_routers;
use serde_json::{json, Value};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

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

// #[tokio::test]
// async fn test_hello() -> Result<()> {
//     let hc = httpc_test::new_client("http://localhost:3000")?;

//     hc.do_get("/").await?.print().await?;
//     hc.do_get("/hello").await?.print().await?;
//     hc.do_get("/hello/what").await?.print().await?;
//     hc.do_get("/hello?name=Mohammad").await?.print().await?;

//     Ok(())
// }

#[tokio::test]
async fn test_groot() -> Result<()> {
    let routers = get_routers().await;

    // `Router` implements `tower::Service<Request<Body>>` so we can
    // call it like any tower service, no need to run an HTTP server.
    let res = routers
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, I'm groot!");

    Ok(())
}

#[tokio::test]
async fn test_say_hello_default() -> Result<()> {
    let mut routers = get_routers().await.into_service();
    let uri = "/hello";
    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut routers)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"<h3> Hello World! </h3>");

    let uris = vec!["/hello", "/hello?names=Mohammad", "/hello?"];
    uris.iter().for_each(|uri| {});

    let uri = "/hello?names=Mohammad";
    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut routers)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"<h3> Hello World! </h3>");

    Ok(())
}

#[tokio::test]
async fn test_say_hello() -> Result<()> {
    let mut routers = get_routers().await.into_service();
    let uri = "/hello";
    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut routers)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"<h3> Hello World! </h3>");

    let uri = "/hello?name=Mohammad";
    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    let response = ServiceExt::<Request<Body>>::ready(&mut routers)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"<h3> Hello Mohammad! </h3>");

    Ok(())
}
