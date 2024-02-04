#[allow(unused)]
use anyhow::{Error, Result};
use serde_json::json;

#[tokio::test]
async fn test_todos() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

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

    hc.do_post("/todo", json!({"text": "Complete this project"}))
        .await?
        .print()
        .await?;

    hc.do_get("/todo").await?.print().await?;

    Ok(())
}
