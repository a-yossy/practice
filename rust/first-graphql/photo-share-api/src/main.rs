use std::sync::LazyLock;

use async_graphql::{
    http::GraphiQLSource, ComplexObject, EmptySubscription, Enum, InputObject, Object, Schema,
    SimpleObject, ID as GraphqlID,
};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use tokio::{net::TcpListener, sync::Mutex};

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

    let mut photos = PHOTOS.lock().await;

    photos.push(Photo {
        id: GraphqlID::from(1),
        name: "Dropping the Heart Chute".to_string(),
        description: Some("The heart chute is one of my favorite chutes".to_string()),
        category: PhotoCategory::Action,
        github_user: GraphqlID::from("gPlake"),
    });
    photos.push(Photo {
        id: GraphqlID::from(2),
        name: "Enjoying the sunshine".to_string(),
        description: None,
        category: PhotoCategory::Selfie,
        github_user: GraphqlID::from("sSchmidt"),
    });
    photos.push(Photo {
        id: GraphqlID::from(3),
        name: "Gunbarrel 25".to_string(),
        description: Some("25 laps on gunbarrel today".to_string()),
        category: PhotoCategory::Landscape,
        github_user: GraphqlID::from("sSchmidt"),
    });

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
