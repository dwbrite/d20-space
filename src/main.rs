mod api;

use axum::{
    Router,
};
use axum::http::StatusCode;

use axum::routing::{get, get_service};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "example_static_file_server=debug,tower_http=debug",
        )
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .fallback(get_service(ServeFile::new("./ui/index.html")).handle_error(|_| async move {unimplemented!()}))
        .nest("/ui", get_service(ServeDir::new("./ui")).handle_error(|_| async move {unimplemented!()}))
        .nest("/api/v0", api::v0::router())
        .layer(TraceLayer::new_for_http());



    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}