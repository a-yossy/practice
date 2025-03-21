use async_graphql::{Context, Error, InputObject, Object, Result, SimpleObject, ID as GraphqlID};
use chrono::Utc;
use mongodb::{bson::doc, Database};

use crate::{
    datetime::DateTime,
    github::{authorize_with_github, GithubAuthorizeResponse, GithubCredential},
    photo::{Photo, PhotoCategory, PhotoDocument},
    random_user::random_user,
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
}
