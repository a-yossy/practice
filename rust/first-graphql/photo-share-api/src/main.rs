use std::sync::LazyLock;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema, ID as GraphqlID};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use chrono::{TimeZone, Utc};
use datetime::DateTime;
use mongodb::{bson::doc, Client, Database};
use mutation::MutationRoot;
use photo::{Photo, PhotoCategory, PHOTOS};
use query::QueryRoot;
use tokio::{net::TcpListener, sync::Mutex};
use user::{User, UserDocument, USERS};

mod datetime;
mod github;
mod mutation;
mod photo;
mod query;
mod random_user;
mod user;

struct Tag {
    photo_id: GraphqlID,
    user_id: GraphqlID,
}

pub static TAGS: LazyLock<Mutex<Vec<Tag>>> = LazyLock::new(|| Mutex::new(Vec::new()));

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
