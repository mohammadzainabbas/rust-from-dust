#[allow(unused)]
use anyhow::{Error, Result};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    hc.Ok(())
}
