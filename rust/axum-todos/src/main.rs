use axum::{
    routing::{delete, get, patch},
    Router,
};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

mod todo;

fn app(pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/todos", get(todo::index).post(todo::create))
        .route("/todos/{id}", delete(todo::delete))
        .route("/todos/{id}/text", patch(todo::update_text))
        .route("/todos/{id}/complete", patch(todo::complete))
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app(pool)).await.unwrap();
}

#[cfg(test)]
mod test {
    use std::net::SocketAddr;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use hyper_util::client::legacy::{connect::HttpConnector, Client};
    use serde_json::{json, Value};
    use sqlx::SqlitePool;
    use tokio::net::TcpListener;

    use crate::todo::Todo;

    use super::*;

    async fn client(pool: SqlitePool) -> (SocketAddr, Client<HttpConnector, Body>) {
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app(pool)).await.unwrap();
        });
        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();

        (addr, client)
    }

    #[sqlx::test(fixtures("todos"))]
    async fn todo_index_200(pool: SqlitePool) {
        let (addr, client) = client(pool.clone()).await;

        let response = client
            .request(
                Request::builder()
                    .method("GET")
                    .uri(format!("http://{addr}/todos"))
                    .header("Host", "localhost")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(
            body,
            json!([
                Todo {
                    id: String::from("ba0232c2-3060-4d30-a680-65e6ed8ebc96"),
                    text: String::from("Rust"),
                    completed: false
                },
                Todo {
                    id: String::from("ba0232c2-3060-4d30-a680-65e6ed8ebc97"),
                    text: String::from("GraphQL"),
                    completed: false
                }
            ])
        );
    }

    #[sqlx::test]
    async fn todo_post_201(pool: SqlitePool) {
        let (addr, client) = client(pool.clone()).await;
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 0);

        let response = client
            .request(
                Request::builder()
                    .method("POST")
                    .uri(format!("http://{addr}/todos"))
                    .header("HOST", "localhost")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({"text": "Rust"}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert!(body["id"].is_string());
        assert!(!body["id"].as_str().unwrap().is_empty());
        assert_eq!(body["text"], json!("Rust"));
        assert_eq!(body["completed"], json!(false));
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos.first().unwrap().text, "Rust");
        assert_eq!(todos.first().unwrap().completed, false);
    }

    #[sqlx::test(fixtures("todos"))]
    async fn todo_update_text_200(pool: SqlitePool) {
        let (addr, client) = client(pool.clone()).await;
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 2);
        let todo_rust = todos.iter().find(|todo| todo.text == String::from("Rust"));
        assert!(todo_rust.is_some());
        let todo_rust = todo_rust.unwrap();
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("GraphQL"))
            .is_some());

        let response = client
            .request(
                Request::builder()
                    .method("PATCH")
                    .uri(format!("http://{addr}/todos/{}/text", todo_rust.id))
                    .header("HOST", "localhost")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({"text": "Rust_update"}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["id"], json!(todo_rust.id));
        assert_eq!(body["text"], json!("Rust_update"));
        assert_eq!(body["completed"], json!(false));
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 2);
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("Rust_update"))
            .is_some());
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("Rust"))
            .is_none());
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("GraphQL"))
            .is_some());
    }

    #[sqlx::test(fixtures("todos"))]
    async fn todo_update_text_404(pool: SqlitePool) {
        let (addr, client) = client(pool.clone()).await;
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 2);
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("Rust"))
            .is_some());
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("GraphQL"))
            .is_some());

        let response = client
            .request(
                Request::builder()
                    .method("PATCH")
                    .uri(format!("http://{addr}/todos/{}/text", "invalid_id"))
                    .header("HOST", "localhost")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({"text": "Rust_update"}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 2);
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("Rust"))
            .is_some());
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("GraphQL"))
            .is_some());
    }

    #[sqlx::test(fixtures("todos"))]
    async fn todo_complete_200(pool: SqlitePool) {
        let (addr, client) = client(pool.clone()).await;
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 2);
        let todo_rust = todos
            .iter()
            .find(|todo| todo.text == String::from("Rust"))
            .unwrap();
        assert_eq!(todo_rust.completed, false);
        assert_eq!(
            todos
                .iter()
                .find(|todo| todo.text == String::from("GraphQL"))
                .unwrap()
                .completed,
            false
        );

        let response = client
            .request(
                Request::builder()
                    .method("PATCH")
                    .uri(format!("http://{addr}/todos/{}/complete", todo_rust.id))
                    .header("HOST", "localhost")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["id"], json!(todo_rust.id));
        assert_eq!(body["text"], json!("Rust"));
        assert_eq!(body["completed"], json!(true));
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 2);
        assert_eq!(
            todos
                .iter()
                .find(|todo| todo.text == String::from("Rust"))
                .unwrap()
                .completed,
            true
        );
        assert_eq!(
            todos
                .iter()
                .find(|todo| todo.text == String::from("GraphQL"))
                .unwrap()
                .completed,
            false
        );
    }

    #[sqlx::test(fixtures("todos"))]
    async fn todo_complete_404(pool: SqlitePool) {
        let (addr, client) = client(pool.clone()).await;
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 2);
        assert_eq!(
            todos
                .iter()
                .find(|todo| todo.text == String::from("Rust"))
                .unwrap()
                .completed,
            false
        );
        assert_eq!(
            todos
                .iter()
                .find(|todo| todo.text == String::from("GraphQL"))
                .unwrap()
                .completed,
            false
        );

        let response = client
            .request(
                Request::builder()
                    .method("PATCH")
                    .uri(format!("http://{addr}/todos/{}/complete", "invalid_id"))
                    .header("HOST", "localhost")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 2);
        assert_eq!(
            todos
                .iter()
                .find(|todo| todo.text == String::from("Rust"))
                .unwrap()
                .completed,
            false
        );
        assert_eq!(
            todos
                .iter()
                .find(|todo| todo.text == String::from("GraphQL"))
                .unwrap()
                .completed,
            false
        );
    }

    #[sqlx::test(fixtures("todos"))]
    async fn todo_delete_204(pool: SqlitePool) {
        let (addr, client) = client(pool.clone()).await;
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 2);
        let todo_rust = todos.iter().find(|todo| todo.text == String::from("Rust"));
        assert!(todo_rust.is_some());
        let todo_rust = todo_rust.unwrap();
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("GraphQL"))
            .is_some());

        let response = client
            .request(
                Request::builder()
                    .method("DELETE")
                    .uri(format!("http://{addr}/todos/{}", todo_rust.id))
                    .header("HOST", "localhost")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 1);
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("Rust"))
            .is_none());
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("GraphQL"))
            .is_some());
    }

    #[sqlx::test(fixtures("todos"))]
    async fn todo_delete_404(pool: SqlitePool) {
        let (addr, client) = client(pool.clone()).await;
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 2);
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("Rust"))
            .is_some());
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("GraphQL"))
            .is_some());

        let response = client
            .request(
                Request::builder()
                    .method("DELETE")
                    .uri(format!("http://{addr}/todos/{}", "invalid_id"))
                    .header("HOST", "localhost")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let todos = sqlx::query_as!(Todo, "SELECT id, text, completed FROM todo")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(todos.len(), 2);
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("Rust"))
            .is_some());
        assert!(todos
            .iter()
            .find(|todo| todo.text == String::from("GraphQL"))
            .is_some());
    }
}
