use crate::api::v0::meta::auth::{AuthCache, Permission, UserAuthorization};
use crate::api::v0::meta::image::Image;
use anyhow::Error;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{AddExtensionLayer, Json, Router};
use s3::Bucket;
use serde::Deserialize;
use serde::Serialize;
use std::io::Cursor;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ImageMapItem {
    Map {
        parent: String,
        child: String,
    },

    Text {
        text: String, // I guess this is just a base64 encoded readme, which means we'll need an API to edit it really... Bleh.
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Map {
    name: String,
    description: String,
    image: String, // bucket object on input, api uri on output
    image_map: Vec<ImageMapItem>,
}

pub fn router(bucket: Arc<Mutex<Bucket>>, auth_cache: Arc<Mutex<AuthCache>>) -> Router {
    let api = Router::new()
        .route("/", get(|| async { "TODO: maps collection" }))
        .route("/:id", get(get_map))
        .route("/:id/image", get(get_map_image));
    api
}

// TODO: proc macros for user-authorization?
async fn get_map(
    user_auth: UserAuthorization,
    Extension(bucket): Extension<Arc<Mutex<Bucket>>>,
    Path(map_id): Path<String>,
) -> Result<Json<Map>, StatusCode> {
    if !user_auth.permissions.contains(&Permission::MapsR) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let (data, _code) = {
        let bucket = bucket.lock().await;
        bucket
            .get_object(format!("/maps/{}/metadata.json", map_id))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let mut map: Map =
        serde_json::from_slice(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    map.image = format!("/api/v0/maps/{}/image", map_id);

    Ok(Json(map))
    // Err(StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_map_image(
    user_auth: UserAuthorization,
    Extension(bucket): Extension<Arc<Mutex<Bucket>>>,
    Path(map_id): Path<String>,
) -> Result<Image, StatusCode> {
    if !user_auth.permissions.contains(&Permission::MapsR) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let (data, _code) = {
        let bucket = bucket.lock().await;
        bucket
            .get_object(format!("/maps/{}/metadata.json", map_id))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let mut map: Map =
        serde_json::from_slice(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (data, _code) = {
        let bucket = bucket.lock().await;
        bucket
            .get_object(format!("/maps/{}/{}", map_id, map.image))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    // let i = image::io::Reader::new(Cursor::new(data));
    let image = Image(data);

    Ok(image)
    // Err(StatusCode::INTERNAL_SERVER_ERROR)
}
