use async_graphql::{Context, Error, InputObject, Object, Result, SimpleObject, ID as GraphqlID};
use chrono::Utc;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};

use crate::{
    datetime::DateTime,
    github::{authorize_with_github, GithubAuthorizeResponse, GithubCredential},
    photo::{Photo, PhotoCategory, PhotoDocument},
    random_user::random_user,
    simple_broker::SimpleBroker,
    tag::TagDocument,
    user::{User, UserDocument},
};

#[derive(InputObject)]
struct PostPhotoInput {
    name: String,
    #[graphql(default_with = "PhotoCategory::Portrait")]
    category: PhotoCategory,
    description: Option<String>,
    tagged_user_ids: Option<Vec<String>>,
}

#[derive(SimpleObject)]
struct AuthPayload {
    token: String,
    user: User,
}

#[derive(InputObject)]
struct TagPhotoWithUsersInput {
    photo_id: String,
    #[graphql(validator(min_items = 1))]
    user_ids: Vec<String>,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn post_photo(&self, ctx: &Context<'_>, input: PostPhotoInput) -> Result<Photo> {
        let current_user = ctx.data::<User>()?;
        let new_photo = PhotoDocument {
            id: None,
            name: input.name,
            description: input.description,
            category: input.category,
            user_id: current_user.github_login.clone().into(),
            created: DateTime(Utc::now()),
        };

        let database = ctx.data::<Database>().unwrap();
        let photo_collection = database.collection::<PhotoDocument>("photos");
        let insert_result = photo_collection.insert_one(&new_photo).await?;
        let insert_id = insert_result
            .inserted_id
            .as_object_id()
            .ok_or("Failed to convert")?;
        if let Some(tagged_user_ids) = input.tagged_user_ids {
            let tag_collection = database.collection::<TagDocument>("tags");
            let new_tags: Vec<TagDocument> = tagged_user_ids
                .into_iter()
                .map(|user_id| TagDocument {
                    photo_id: insert_id.to_string(),
                    user_id,
                })
                .collect();
            tag_collection.insert_many(&new_tags).await?;
        }
        let photo = Photo {
            id: insert_id.into(),
            name: new_photo.name,
            description: new_photo.description,
            category: new_photo.category,
            github_user: new_photo.user_id.into(),
            created: new_photo.created,
        };
        SimpleBroker::publish(photo.clone());
        Ok(photo)
    }

    async fn github_auth(&self, ctx: &Context<'_>, code: String) -> Result<AuthPayload> {
        let client_id = dotenv::var("CLIENT_ID").unwrap();
        let client_secret = dotenv::var("CLIENT_SECRET").unwrap();
        let GithubAuthorizeResponse {
            message,
            access_token,
            avatar_url,
            login,
            name,
        } = authorize_with_github(GithubCredential {
            client_id,
            client_secret,
            code,
        })
        .await;

        if let Some(err_msg) = message {
            return Err(Error::new(err_msg));
        }

        let latest_user_info = UserDocument {
            name,
            github_login: login,
            github_token: access_token,
            avatar: avatar_url,
        };

        let database = ctx.data::<Database>().unwrap();
        let collection = database.collection::<UserDocument>("users");
        collection
            .replace_one(
                doc! {"github_login": &latest_user_info.github_login},
                &latest_user_info,
            )
            .upsert(true)
            .await
            .unwrap();

        Ok(AuthPayload {
            token: latest_user_info.github_token,
            user: User {
                github_login: latest_user_info.github_login.into(),
                name: latest_user_info.name,
                avatar: latest_user_info.avatar,
            },
        })
    }

    async fn add_fake_users(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 1)] count: usize,
    ) -> Result<Vec<User>> {
        let response = random_user(count).await;
        let new_users = response
            .results
            .iter()
            .map(|result| UserDocument {
                github_login: result.login.username.clone(),
                name: Some(format!("{} {}", result.name.first, result.name.last)),
                avatar: Some(result.picture.thumbnail.clone()),
                github_token: result.login.sha1.clone(),
            })
            .collect::<Vec<UserDocument>>();
        let database = ctx.data::<Database>().unwrap();
        let collection = database.collection::<UserDocument>("users");
        collection.insert_many(&new_users).await?;

        Ok(new_users
            .iter()
            .map(|user| User {
                name: user.name.clone(),
                avatar: user.avatar.clone(),
                github_login: user.github_login.clone().into(),
            })
            .collect::<Vec<User>>())
    }

    async fn fake_user_auth(
        &self,
        ctx: &Context<'_>,
        github_login: GraphqlID,
    ) -> Result<AuthPayload> {
        let database = ctx.data::<Database>().unwrap();
        let collection = database.collection::<UserDocument>("users");
        let user = collection
            .find_one(doc! {"github_login": github_login.to_string()})
            .await
            .unwrap();
        if let Some(user) = user {
            Ok(AuthPayload {
                token: user.github_token,
                user: User {
                    github_login,
                    name: user.name,
                    avatar: user.avatar,
                },
            })
        } else {
            Err(Error::new("Cannot find user"))
        }
    }

    async fn tag_photo_with_users(
        &self,
        ctx: &Context<'_>,
        input: TagPhotoWithUsersInput,
    ) -> Result<Photo> {
        let database = ctx.data::<Database>().unwrap();
        let photo_collection = database.collection::<PhotoDocument>("photos");
        let photo = photo_collection
            .find_one(doc! {"_id": ObjectId::parse_str(&input.photo_id).unwrap()})
            .await
            .unwrap()
            .ok_or("photo not found")?;
        let user_collection = database.collection::<UserDocument>("users");
        let user_count = user_collection
            .count_documents(doc! { "github_login": { "$in": &input.user_ids }})
            .await?;
        if input.user_ids.len() != user_count as usize {
            return Err(Error::new("user not found"));
        }
        let tag_collection = database.collection::<TagDocument>("tags");
        tag_collection
            .delete_many(doc! {"photo_id": &input.photo_id})
            .await?;
        let new_tags: Vec<TagDocument> = input
            .user_ids
            .into_iter()
            .map(|user_id| TagDocument {
                photo_id: input.photo_id.clone(),
                user_id,
            })
            .collect();
        tag_collection.insert_many(&new_tags).await?;
        let photo = Photo {
            id: photo.id.unwrap().into(),
            name: photo.name,
            description: photo.description,
            category: photo.category,
            github_user: photo.user_id.into(),
            created: photo.created,
        };
        Ok(photo)
    }
}
