mod api;

use axum::body::HttpBody;
use axum::http::{Request, Response, StatusCode};
use axum::response::{IntoResponse, Redirect};
use axum::Router;
use std::convert::Infallible;
use std::io;

use axum::routing::{get, get_service};
use tower::{Service, ServiceBuilder};
use tower_http::services::fs::ServeFileSystemResponseBody;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .fallback(
            get_service(ServeFile::new("./target/ui/index.html"))
                .handle_error(|_: io::Error| async move { unimplemented!() }),
        )
        .nest(
            "/static",
            get_service(ServeDir::new("./target/ui"))
                .handle_error(|_: io::Error| async move { unimplemented!() }),
        )
        .nest("/api/v0", api::v0::router())
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
