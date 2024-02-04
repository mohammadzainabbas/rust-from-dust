#[allow(unused)]
use anyhow::{Error, Result};
use serde_json::json;

#[tokio::test]
async fn test_todos() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    //! ZOMBIE
    //! Z - zero
    //! O - one
    //! M - many
    //! B - boundary conditions
    //! I - 
    //! E - exceptions

    hc.do_get("/todo").await?.print().await?;
    hc.do_post("/todo", json!({"text": "Complete this project"}))
        .await?
        .print()
        .await?;
    Ok(())
}
