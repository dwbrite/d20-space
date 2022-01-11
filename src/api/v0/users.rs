use std::sync::Arc;
use axum::{Json, Router};
use axum::extract::Extension;
use s3::Bucket;
use tokio::sync::Mutex;
use crate::api::v0::meta::auth::UserAuthorization;
use serde::{Serialize, Deserialize};
use axum::routing::*;

pub fn router() -> Router {
    let api = Router::new()
        .route("/", get(list_users))
        .route("/authenticate", post(authenticate));
    api
}

pub async fn authenticate(user_auth: UserAuthorization) -> Json<UserAuthorization> {
    Json(user_auth)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
    // TODO: groups (permissions)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Users {
    users: Vec<User>
}

pub async fn list_users(Extension(bucket): Extension<Arc<Mutex<Bucket>>>) -> Json<Users>{
    let bucket = bucket.lock().await;
    let response = bucket.list("players/".to_string(), Some("/".to_string())).await.unwrap();

    let mut users = Users {
        users: vec![]
    };

    // TODO: cache

    for prefix in response.first().unwrap().common_prefixes.clone().unwrap() {
        users.users.push(User { name: prefix.prefix.split("/").collect::<Vec<&str>>()[1].to_string() }) ;
    }

    println!("{:?}", response);



    Json(users)
}