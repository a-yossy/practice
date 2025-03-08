use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn plain_text() -> &'static str {
    "foo"
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/about", get(|| async { "About" }))
        .route("/plain_text", get(plain_text))
        .route("/json", get(json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
