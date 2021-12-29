use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    let api = Router::new()
        .route("/", get(|| async { "Fuck off, it's not done yet." }));
    api
}