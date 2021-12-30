use anyhow::Error;
use async_trait::async_trait;
use axum::extract::{Extension, TypedHeader};
use axum::headers::authorization::Basic;
use axum::headers::Authorization;
use axum::http::StatusCode;
use axum::{
    extract::{extractor_middleware, FromRequest, RequestParts},
    http,
    routing::{get, post},
    Router,
};
use s3::Bucket;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::join;
use tokio::sync::Mutex;
use tower_http::classify::GrpcCode::Unauthenticated;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    MapsR, // This could probably even include the list of maps accessible
    SelfRW,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAuthorization {
    pub identity: String,
    pub permissions: Vec<Permission>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub pin: String,
    pub permissions: Vec<Permission>,
}

pub struct AuthCache {
    pub cache: HashMap<String, AuthData>,
}

impl AuthCache {
    async fn load_from_bucket(&mut self, bucket: &mut Bucket, player: &str) -> anyhow::Result<()> {
        let (data, code) = bucket
            .get_object(format!("/players/{}/auth.json", player))
            .await?;

        if code != 200 {
            return Err(Error::msg("Probably failed to get that ish."));
        }

        // unwrap because this _should_ be infallible if we get a response
        let auth_data: AuthData = serde_json::from_slice(&data).unwrap();
        self.cache.insert(player.to_string(), auth_data);
        Ok(())
    }

    async fn try_authenticate(
        &mut self,
        player: &str,
        pin: &str,
    ) -> Result<UserAuthorization, StatusCode> {
        if let Some(auth_data) = self.cache.get(&player.to_string()) {
            if pin == auth_data.pin {
                return Ok(UserAuthorization {
                    identity: player.to_string(),
                    permissions: auth_data.permissions.clone(),
                });
            }
        }

        Err(StatusCode::UNAUTHORIZED)
    }
}

#[async_trait]
impl<B> FromRequest<B> for UserAuthorization
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(auth_cache) = Extension::<Arc<Mutex<AuthCache>>>::from_request(req)
            .await
            .unwrap();

        let Extension(bucket) = Extension::<Arc<Mutex<Bucket>>>::from_request(req)
            .await
            .unwrap();

        let TypedHeader(Authorization(basic_auth)) =
            TypedHeader::<axum::headers::Authorization<Basic>>::from_request(req)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let (player, pin) = (basic_auth.username(), basic_auth.password());

        let (mut auth_cache, mut bucket) = join!(auth_cache.lock(), bucket.lock());

        // try authenticate. On fail: load from bucket then try again
        return if let Ok(user_auth) = auth_cache.try_authenticate(player, pin).await {
            Ok(user_auth)
        } else if let Ok(_) = auth_cache.load_from_bucket(&mut bucket, player).await {
            auth_cache.try_authenticate(player, pin).await
        } else {
            Err(StatusCode::UNAUTHORIZED)
        };
    }
}
