use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct Todo {
    id: String,
    text: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}

async fn todos_index(State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    let todos = sqlx::query_as!(
        Todo,
        r#"
            SELECT
                id,
                text,
                completed != 0 AS "completed: bool"
            FROM
                todo
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(todos)
}

async fn todo_create(
    State(pool): State<Pool<Sqlite>>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let todo = sqlx::query_as!(
        Todo,
        r#"
            INSERT INTO
                todo (id, text)
            VALUES
                ($1, $2)
            RETURNING
                id,
                text,
                completed != 0 AS "completed: bool"
        "#,
        id,
        input.text
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(todo))
}

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let app = Router::new()
        .route("/todos", get(todos_index))
        .route("/todos", post(todo_create))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
