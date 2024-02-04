use tracing::{trace, Level};
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
/// Fn [`setup_tracing`] creates three basic routes, returns a [`axum::Router`](https://docs.rs/axum/latest/axum/struct.Router.html) object.
///
/// - `/` with [`axum::routing::get`](https://docs.rs/axum/latest/axum/routing/method_routing/fn.get.html) method
/// - `/hello` with [`axum::routing::get`](https://docs.rs/axum/latest/axum/routing/method_routing/fn.get.html) method
/// - `/hello/:path` with [`axum::routing::get`](https://docs.rs/axum/latest/axum/routing/method_routing/fn.get.html) method
///
pub async fn setup_tracing() {
    let log_dir = "./logs";
    let debug_file = rolling::daily(log_dir, "debug").with_min_level(Level::WARN);
    let warning_file = rolling::daily(log_dir, "warning").with_max_level(Level::WARN);
    let log_files = debug_file.and(warning_file);

    tracing_subscriber::fmt()
        .json()
        .with_writer(log_files)
        .with_ansi(true)
        .with_max_level(Level::TRACE)
        .init();

    trace!("setup_tracing() done!");
}
