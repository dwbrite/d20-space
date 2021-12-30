pub mod auth;
mod players;

use crate::api::v0::auth::{AuthCache, Permission, UserAuthorization};
use axum::extract::Extension;
use axum::routing::*;
use axum::{AddExtensionLayer, Json, Router};
use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn configure_bucket() -> Bucket {
    let name = env!("BUCKET_NAME").to_string();
    let region = Region::Custom {
        region: env!("BUCKET_REGION").to_string(),
        endpoint: env!("BUCKET_ENDPOINT").to_string(),
    };
    let credentials = Credentials {
        access_key: Some(env!("BUCKET_ACCESS").to_string()),
        secret_key: Some(env!("BUCKET_SECRET").to_string()),
        security_token: None,
        session_token: None,
    };

    Bucket::new(&name, region, credentials).unwrap()
}

pub fn router() -> Router {
    let bucket = Arc::new(Mutex::new(configure_bucket()));
    let auth_cache = Arc::new(Mutex::new(AuthCache {
        cache: HashMap::new(),
    }));

    let api = Router::new()
        .route("/", get(|| async { "Fuck off, it's not done yet." }))
        .route("/map", get(get_map))
        .layer(AddExtensionLayer::new(auth_cache))
        .layer(AddExtensionLayer::new(bucket));
    api
}

// TODO: proc macro for user-authorization?
async fn get_map(user_auth: UserAuthorization) -> Json<Value> {
    if !user_auth.permissions.contains(&Permission::Map) {
        // TODO: return unauthorized
        return Json(json!({"ps": "you suck"}));
    }

    Json(json!({"ps": "you're great!'"}))
}
