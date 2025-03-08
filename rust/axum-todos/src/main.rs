use axum::{
    routing::{delete, get, patch},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;

mod todo;

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let app = Router::new()
        .route("/todos", get(todo::index).post(todo::create))
        .route("/todos/{id}", delete(todo::delete))
        .route("/todos/{id}/text", patch(todo::update_text))
        .route("/todos/{id}/complete", patch(todo::complete))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
