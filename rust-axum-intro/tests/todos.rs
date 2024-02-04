#[allow(unused)]
use anyhow::{Error, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/").await?.print().await?;
    // hc.do_get("/hello").await?.print().await?;
    // hc.do_get("/hello/what").await?.print().await?;
    // hc.do_get("/hello?name=Mohammad").await?.print().await?;

    hc.do_get("/todo").await?.print().await?;
    hc.do_post("/todo", json!({"text": "Complete this project"}))
        .await?
        .print()
        .await?;
    Ok(())
}
