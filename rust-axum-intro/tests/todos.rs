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

use anyhow::Result;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect`
use rust_axum_intro::get_routers;
use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

#[tokio::test]
async fn test_todo() -> Result<(), anyhow::Error> {
    Ok(())
}
