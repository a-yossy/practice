use async_graphql::{ComplexObject, Context, Enum, Error, Result, SimpleObject, ID as GraphqlID};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use serde::{Deserialize, Serialize};

use crate::{
    datetime::DateTime,
    tag::TagDocument,
    user::{User, UserDocument},
};

#[derive(Enum, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhotoCategory {
    Selfie,
    Portrait,
    Action,
    Landscape,
    Graphic,
}

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

    async fn tagged_users(&self, ctx: &Context<'_>) -> Vec<User> {
        let database = ctx.data::<Database>().unwrap();
        let tag_collection = database.collection::<TagDocument>("tags");
        let mut tag_cursor = tag_collection
            .find(doc! { "photo_id": self.id.to_string() })
            .await
            .unwrap();
        let mut user_ids = Vec::new();
        while let Some(tag) = tag_cursor.try_next().await.unwrap() {
            user_ids.push(tag.user_id);
        }
        let user_collection = database.collection::<UserDocument>("users");
        let mut user_cursor = user_collection
            .find(doc! { "github_login": { "$in": user_ids } })
            .await
            .unwrap();
        let mut users = Vec::new();
        while let Some(user) = user_cursor.try_next().await.unwrap() {
            users.push(User {
                github_login: user.github_login.into(),
                name: user.name,
                avatar: user.avatar,
            });
        }
        users
    }
}
