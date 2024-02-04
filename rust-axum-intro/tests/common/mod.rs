use axum::Router;
use rust_axum_intro;

pub fn setup() -> Router {
    rust_axum_intro::get_routers()
}
