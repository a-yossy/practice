use std::sync::LazyLock;

use async_graphql::{ComplexObject, SimpleObject, ID as GraphqlID};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
    photo::{Photo, PHOTOS},
    TAGS,
};

pub static USERS: LazyLock<Mutex<Vec<User>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Serialize, Deserialize)]
pub struct UserDocument {
    pub github_login: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub github_token: String,
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct User {
    pub github_login: GraphqlID,
    pub name: Option<String>,
    pub avatar: Option<String>,
}

#[ComplexObject]
impl User {
    async fn posted_photos(&self) -> Vec<Photo> {
        PHOTOS
            .lock()
            .await
            .iter()
            .filter(|photo| photo.github_user == self.github_login)
            .cloned()
            .collect()
    }

    async fn in_photos(&self) -> Vec<Photo> {
        let tags = TAGS.lock().await;
        let photos = PHOTOS.lock().await;
        tags.iter()
            .filter(|tag| tag.user_id == self.github_login)
            .map(|tag| &tag.photo_id)
            .map(|photo_id| photos.iter().find(|photo| photo.id == *photo_id).unwrap())
            .cloned()
            .collect()
    }
}
