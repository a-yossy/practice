use std::sync::LazyLock;

use async_graphql::{
    http::GraphiQLSource, ComplexObject, EmptySubscription, Enum, InputObject, InputValueError,
    Object, Scalar, ScalarType, Schema, SimpleObject, Value, ID as GraphqlID,
};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use chrono::{DateTime as ChronoDateTime, TimeZone, Utc};
use tokio::{net::TcpListener, sync::Mutex};

#[derive(Clone)]
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

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
enum PhotoCategory {
    Selfie,
    Portrait,
    Action,
    Landscape,
    Graphic,
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

    async fn posted_by(&self) -> User {
        USERS
            .lock()
            .await
            .iter()
            .find(|user| user.github_login == self.github_user)
            .unwrap()
            .clone()
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
static ID: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn total_photos(&self) -> usize {
        let photos = PHOTOS.lock().await;
        photos.len()
    }

    async fn all_photos(&self) -> Vec<Photo> {
        let photos = PHOTOS.lock().await;
        photos.to_vec()
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn post_photo(&self, input: PostPhotoInput) -> Photo {
        let mut photos = PHOTOS.lock().await;
        let mut id = ID.lock().await;
        *id += 1;
        let new_photo = Photo {
            id: GraphqlID::from(id),
            name: input.name,
            description: input.description,
            category: input.category,
            github_user: GraphqlID::from("mHattrup"),
            created: DateTime(Utc::now()),
        };
        photos.push(new_photo.clone());

        new_photo
    }
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() {
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

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
