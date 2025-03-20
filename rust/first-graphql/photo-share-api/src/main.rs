use std::sync::LazyLock;

use async_graphql::{
    http::GraphiQLSource, ComplexObject, Context, EmptySubscription, Enum, Error, InputObject,
    InputValueError, Object, Result, Scalar, ScalarType, Schema, SimpleObject, Value,
    ID as GraphqlID,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use chrono::{DateTime as ChronoDateTime, TimeZone, Utc};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Database,
};
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::Mutex};

#[derive(Clone, Serialize, Deserialize)]
struct DateTime(ChronoDateTime<Utc>);

#[Scalar]
impl ScalarType for DateTime {
    fn parse(value: Value) -> async_graphql::InputValueResult<Self> {
        if let Value::String(value) = &value {
            let date_time = value
                .parse::<ChronoDateTime<Utc>>()
                .map_err(|e| InputValueError::custom(format!("無効な DateTime: {}", e)))?;
            Ok(DateTime(date_time))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }
}

#[derive(Enum, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum PhotoCategory {
    Selfie,
    Portrait,
    Action,
    Landscape,
    Graphic,
}

#[derive(Serialize, Deserialize)]
struct PhotoDocument {
    #[serde(rename = "_id", skip_serializing)]
    id: Option<ObjectId>,
    name: String,
    description: Option<String>,
    category: PhotoCategory,
    user_id: String,
    created: DateTime,
}

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
struct Photo {
    id: GraphqlID,
    name: String,
    description: Option<String>,
    category: PhotoCategory,
    #[graphql(skip)]
    github_user: GraphqlID,
    created: DateTime,
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

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
struct User {
    github_login: GraphqlID,
    name: Option<String>,
    avatar: Option<String>,
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

struct Tag {
    photo_id: GraphqlID,
    user_id: GraphqlID,
}

#[derive(InputObject)]
struct PostPhotoInput {
    name: String,
    #[graphql(default_with = "PhotoCategory::Portrait")]
    category: PhotoCategory,
    description: Option<String>,
}

static PHOTOS: LazyLock<Mutex<Vec<Photo>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static USERS: LazyLock<Mutex<Vec<User>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static TAGS: LazyLock<Mutex<Vec<Tag>>> = LazyLock::new(|| Mutex::new(Vec::new()));

struct QueryRoot;

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

    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        ctx.data::<User>().cloned()
    }
}

#[derive(Serialize)]
struct GithubCredential {
    client_id: String,
    client_secret: String,
    code: String,
}

#[derive(Deserialize)]
struct GithubCredentialResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GithubUserAccountResponse {
    message: Option<String>,
    avatar_url: Option<String>,
    login: String,
    name: Option<String>,
}

struct GithubAuthorizeResponse {
    access_token: String,
    message: Option<String>,
    avatar_url: Option<String>,
    login: String,
    name: Option<String>,
}

#[derive(SimpleObject)]
struct AuthPayload {
    token: String,
    user: User,
}

#[derive(Serialize, Deserialize)]
struct UserDocument {
    github_login: String,
    name: Option<String>,
    avatar: Option<String>,
    github_token: String,
}

async fn request_github_token(credential: GithubCredential) -> GithubCredentialResponse {
    let client = ReqwestClient::new();
    client
        .post("https://github.com/login/oauth/access_token")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&credential)
        .send()
        .await
        .unwrap()
        .json::<GithubCredentialResponse>()
        .await
        .unwrap()
}

async fn request_github_user_account(token: &String) -> GithubUserAccountResponse {
    let client = ReqwestClient::new();
    client
        .get("https://api.github.com/user")
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "rust-first-graphql")
        .send()
        .await
        .unwrap()
        .json::<GithubUserAccountResponse>()
        .await
        .unwrap()
}

async fn authorize_with_github(credential: GithubCredential) -> GithubAuthorizeResponse {
    let GithubCredentialResponse { access_token } = request_github_token(credential).await;
    let GithubUserAccountResponse {
        message,
        avatar_url,
        login,
        name,
    } = request_github_user_account(&access_token).await;

    GithubAuthorizeResponse {
        access_token,
        message,
        avatar_url,
        login,
        name,
    }
}

#[derive(Deserialize)]
struct RandomUserLoginResponse {
    username: String,
    sha1: String,
}
#[derive(Deserialize)]
struct RandomUserNameResponse {
    first: String,
    last: String,
}
#[derive(Deserialize)]
struct RandomUserPictureResponse {
    thumbnail: String,
}
#[derive(Deserialize)]
struct RandomUserResponseBody {
    login: RandomUserLoginResponse,
    name: RandomUserNameResponse,
    picture: RandomUserPictureResponse,
}
#[derive(Deserialize)]
struct RandomUserResponse {
    results: Vec<RandomUserResponseBody>,
}

struct MutationRoot;

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
        let collection = database.collection::<PhotoDocument>("photos");
        let insert_result = collection.insert_one(&new_photo).await?;
        let insert_id = insert_result
            .inserted_id
            .as_object_id()
            .ok_or("Failed to convert")?;
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
        let random_user_api = format!("https://randomuser.me/api/?results={}", count);
        let client = ReqwestClient::new();
        let results = client
            .get(random_user_api)
            .send()
            .await
            .unwrap()
            .json::<RandomUserResponse>()
            .await
            .unwrap();
        let new_users = results
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
}

async fn graphql_handler(
    State(AppState { schema, database }): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    if let Some(token) = get_token_from_headers(&headers) {
        let collection = database.collection::<UserDocument>("users");
        let user = collection.find_one(doc! {"github_token": token.0}).await;
        if let Ok(user) = user {
            if let Some(user) = user {
                let user = User {
                    name: user.name,
                    avatar: user.avatar,
                    github_login: user.github_login.into(),
                };
                request = request.data(user);
            }
        }
    }
    schema.execute(request).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().finish())
}

struct Token(String);

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

#[derive(Clone)]
struct AppState {
    schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
    database: Database,
}

#[tokio::main]
async fn main() {
    let uri = dotenv::var("DB_HOST").unwrap();
    let client = Client::with_uri_str(uri).await.unwrap();
    let database = client.database("sample");

    {
        let mut users = USERS.lock().await;
        users.push(User {
            github_login: GraphqlID::from("mHattrup"),
            name: Some("Mike Hattrup".to_string()),
            avatar: None,
        });
        users.push(User {
            github_login: GraphqlID::from("gPlake"),
            name: Some("Glen Plake".to_string()),
            avatar: None,
        });
        users.push(User {
            github_login: GraphqlID::from("sSchmidt"),
            name: Some("Scot Schmidt".to_string()),
            avatar: None,
        });
    }

    {
        let mut photos = PHOTOS.lock().await;
        photos.push(Photo {
            id: GraphqlID::from(1),
            name: "Dropping the Heart Chute".to_string(),
            description: Some("The heart chute is one of my favorite chutes".to_string()),
            category: PhotoCategory::Action,
            github_user: GraphqlID::from("gPlake"),
            created: DateTime(Utc.with_ymd_and_hms(1997, 3, 28, 0, 0, 0).unwrap()),
        });
        photos.push(Photo {
            id: GraphqlID::from(2),
            name: "Enjoying the sunshine".to_string(),
            description: None,
            category: PhotoCategory::Selfie,
            github_user: GraphqlID::from("sSchmidt"),
            created: DateTime(Utc.with_ymd_and_hms(1985, 2, 1, 0, 0, 0).unwrap()),
        });
        photos.push(Photo {
            id: GraphqlID::from(3),
            name: "Gunbarrel 25".to_string(),
            description: Some("25 laps on gunbarrel today".to_string()),
            category: PhotoCategory::Landscape,
            github_user: GraphqlID::from("sSchmidt"),
            created: DateTime(Utc.with_ymd_and_hms(2018, 4, 15, 19, 9, 57).unwrap()),
        });
    }

    {
        let mut tags = TAGS.lock().await;
        tags.push(Tag {
            photo_id: "1".into(),
            user_id: "gPlake".into(),
        });
        tags.push(Tag {
            photo_id: "2".into(),
            user_id: "sSchmidt".into(),
        });
        tags.push(Tag {
            photo_id: "2".into(),
            user_id: "mHattrup".into(),
        });
        tags.push(Tag {
            photo_id: "2".into(),
            user_id: "gPlake".into(),
        });
    }

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(database.clone())
        .finish();
    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .with_state(AppState { schema, database });
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
