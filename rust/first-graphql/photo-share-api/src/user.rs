use async_graphql::{ComplexObject, Context, SimpleObject, ID as GraphqlID};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use serde::{Deserialize, Serialize};

use crate::{
    photo::{Photo, PhotoDocument},
    tag::TagDocument,
};

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
    async fn posted_photos(&self, ctx: &Context<'_>) -> Vec<Photo> {
        let database = ctx.data::<Database>().unwrap();
        let collection = database.collection::<PhotoDocument>("photos");
        let mut cursor = collection.find(doc! {}).await.unwrap();
        let mut photos = Vec::new();
        while let Some(photo) = cursor.try_next().await.unwrap() {
            photos.push(Photo {
                id: photo.id.unwrap().into(),
                name: photo.name,
                description: photo.description,
                category: photo.category,
                github_user: photo.user_id.into(),
                created: photo.created,
            });
        }
        photos
            .iter()
            .filter(|photo| photo.github_user == self.github_login)
            .cloned()
            .collect()
    }

    async fn in_photos(&self, ctx: &Context<'_>) -> Vec<Photo> {
        let database = ctx.data::<Database>().unwrap();
        let tag_collection = database.collection::<TagDocument>("tags");
        let mut tag_cursor = tag_collection
            .find(doc! { "user_id": self.github_login.to_string() })
            .await
            .unwrap();
        let mut photo_ids = Vec::new();
        while let Some(tag) = tag_cursor.try_next().await.unwrap() {
            photo_ids.push(ObjectId::parse_str(tag.photo_id).unwrap());
        }
        let photo_collection = database.collection::<PhotoDocument>("photos");
        let mut photo_cursor = photo_collection
            .find(doc! { "_id": { "$in": photo_ids } })
            .await
            .unwrap();
        let mut photos = Vec::new();
        while let Some(photo) = photo_cursor.try_next().await.unwrap() {
            photos.push(Photo {
                id: photo.id.unwrap().into(),
                name: photo.name,
                description: photo.description,
                category: photo.category,
                github_user: photo.user_id.into(),
                created: photo.created,
            });
        }
        photos
    }
}
