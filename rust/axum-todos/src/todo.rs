use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub completed: bool,
}

pub async fn index(State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    let todos = sqlx::query_as!(
        Todo,
        r#"
SELECT
    id,
    text,
    completed
FROM
    todo
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(todos)
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    text: String,
}
pub async fn create(
    State(pool): State<Pool<Sqlite>>,
    Json(params): Json<CreateTodo>,
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
    completed
        "#,
        id,
        params.text
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(todo))
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoText {
    text: String,
}
pub async fn update_text(
    Path(id): Path<String>,
    State(pool): State<Pool<Sqlite>>,
    Json(params): Json<UpdateTodoText>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
UPDATE
    todo
SET
    text = $1
WHERE
    id = $2
RETURNING
    id,
    text,
    completed
        "#,
        params.text,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

pub async fn complete(
    Path(id): Path<String>,
    State(pool): State<Pool<Sqlite>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
UPDATE
    todo
SET
    completed = true
WHERE
    id = $1
RETURNING
    id,
    text,
    completed
        "#,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

pub async fn delete(Path(id): Path<String>, State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
DELETE FROM
    todo
WHERE
    id = $1
        "#,
        id
    )
    .execute(&pool)
    .await;

    if result.is_ok() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
