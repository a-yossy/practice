use std::sync::LazyLock;

use async_graphql::{ComplexObject, Context, Enum, Error, Result, SimpleObject, ID as GraphqlID};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
    datetime::DateTime,
    user::{User, UserDocument, USERS},
    TAGS,
};

#[derive(Enum, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhotoCategory {
    Selfie,
    Portrait,
    Action,
    Landscape,
    Graphic,
}

pub static PHOTOS: LazyLock<Mutex<Vec<Photo>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Serialize, Deserialize)]
pub struct PhotoDocument {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: Option<String>,
    pub category: PhotoCategory,
    pub user_id: String,
    pub created: DateTime,
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Photo {
    pub id: GraphqlID,
    pub name: String,
    pub description: Option<String>,
    pub category: PhotoCategory,
    #[graphql(skip)]
    pub github_user: GraphqlID,
    pub created: DateTime,
}

#[ComplexObject]
impl Photo {
    async fn url(&self) -> String {
        format!("https://yoursite.com/img/{}.jpg", *self.id)
    }

    async fn posted_by(&self, ctx: &Context<'_>) -> Result<User> {
        let database = ctx.data::<Database>().unwrap();
        let collection = database.collection::<UserDocument>("users");
        let user = collection
            .find_one(doc! {"github_login": self.github_user.to_string()})
            .await?;
        if let Some(user) = user {
            let user = User {
                name: user.name,
                avatar: user.avatar,
                github_login: user.github_login.into(),
            };
            Ok(user)
        } else {
            Err(Error::new("Not found user"))
        }
    }

    async fn tagged_users(&self) -> Vec<User> {
        let tags = TAGS.lock().await;
        let users = USERS.lock().await;
        tags.iter()
            .filter(|tag| tag.photo_id == self.id)
            .map(|tag| &tag.user_id)
            .map(|user_id| {
                users
                    .iter()
                    .find(|user| user.github_login == *user_id)
                    .unwrap()
            })
            .cloned()
            .collect()
    }
}
