mod api;

use std::convert::Infallible;
use std::io;
use axum::{
    Router,
};
use axum::body::HttpBody;
use axum::http::{Response, StatusCode};
use axum::response::{IntoResponse, Redirect};

use axum::routing::{get, get_service};
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::services::fs::ServeFileSystemResponseBody;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .fallback(get_service(
            ServiceBuilder::new()
                .and_then(|response: Response<ServeFileSystemResponseBody>| async move {
                    let response = if response.status() == StatusCode::NOT_FOUND {
                        Redirect::to("/".parse().unwrap()).into_response()
                    }else {
                        response.map(|body| body.boxed()).into_response()
                    };
                    Ok::<_, _>(response)
                })
                .service(ServeDir::new("./ui"))
            // ServeDir::new("./ui")
        ).handle_error(|_: io::Error| async move {unimplemented!()})
        )
        .nest("/api/v0", api::v0::router())
        .layer(TraceLayer::new_for_http());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}