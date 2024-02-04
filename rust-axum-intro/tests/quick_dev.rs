#[allow(unused)]
use anyhow::{Error, Result};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/").await?.print().await?;
    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello/What's Up").await?.print().await?;
    hc.do_get("/hello?name=Mohammad").await?.print().await?;

    Ok(())
}
