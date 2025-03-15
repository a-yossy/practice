use std::sync::Mutex;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Object, Schema, SimpleObject};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[derive(SimpleObject, Clone)]
struct Photo {
    id: String,
    name: String,
    description: Option<String>,
}

static PHOTOS: Mutex<Vec<Photo>> = Mutex::new(Vec::new());
static ID: Mutex<usize> = Mutex::new(0);

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn total_photos(&self) -> usize {
        let photos = PHOTOS.lock().unwrap();
        photos.len()
    }

    async fn all_photos(&self) -> Vec<Photo> {
        let photos = PHOTOS.lock().unwrap();
        photos.to_vec()
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn post_photo(&self, name: String, description: Option<String>) -> Photo {
        let mut photos = PHOTOS.lock().unwrap();
        let mut id = ID.lock().unwrap();
        *id += 1;
        let new_photo = Photo {
            id: id.to_string(), name, description
        };
        photos.push(new_photo.clone());

        new_photo
    }
}

async fn grashiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
    let app = Router::new().route("/", get(grashiql).post_service(GraphQL::new(schema)));
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
