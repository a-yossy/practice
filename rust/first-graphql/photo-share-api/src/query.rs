use async_graphql::{Context, Object};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Database};

use crate::{
    photo::{Photo, PhotoDocument},
    user::{User, UserDocument},
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn total_photos(&self, ctx: &Context<'_>) -> usize {
        let database = ctx.data::<Database>().unwrap();
        let photos = database.collection::<PhotoDocument>("photos");
        photos.count_documents(doc! {}).await.unwrap() as usize
    }

    async fn all_photos(&self, ctx: &Context<'_>) -> Vec<Photo> {
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
    }

    async fn total_users(&self, ctx: &Context<'_>) -> usize {
        let database = ctx.data::<Database>().unwrap();
        let users = database.collection::<Vec<UserDocument>>("users");
        users.count_documents(doc! {}).await.unwrap() as usize
    }

    async fn all_users(&self, ctx: &Context<'_>) -> Vec<User> {
        let database = ctx.data::<Database>().unwrap();
        let collection = database.collection::<UserDocument>("users");
        let mut cursor = collection.find(doc! {}).await.unwrap();
        let mut users = Vec::new();
        while let Some(user) = cursor.try_next().await.unwrap() {
            users.push(User {
                name: user.name,
                github_login: user.github_login.into(),
                avatar: user.avatar,
            });
        }

        users
    }

    async fn me(&self, ctx: &Context<'_>) -> Option<User> {
        ctx.data::<User>().ok().map(|user| user.clone())
    }
}
