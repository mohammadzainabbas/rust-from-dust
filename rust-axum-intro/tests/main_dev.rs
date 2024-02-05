#![allow(unused_imports)]

use anyhow::Result;
use axum::http::response;
use rust_axum_intro::get_routers;
use tower::{Service, ServiceExt};

#[tokio::test]
async fn test_main() -> Result<()> {
    let routers = get_routers().await;

    let res = routers.oneshot(req)

    Ok(())
}
