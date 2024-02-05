#![allow(unused_imports)]

use anyhow::Result;
use axum::{
    body::Body,
    http::{response, Request},
};
use rust_axum_intro::get_routers;
use tower::{Service, ServiceExt};

#[tokio::test]
async fn test_main() -> Result<()> {
    let routers = get_routers().await;

    let res = routers.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap());

    Ok(())
}
