// #[allow(unused)]
use anyhow::{Error, Result};

use rust_axum_intro;

#[tokio::test]
async fn test_main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/").await?.print().await?;

    Ok(())
}