use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use mongodb::{bson::doc, Client, Database};
use mutation::MutationRoot;
use query::QueryRoot;
use subscription::SubscriptionRoot;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use user::{User, UserDocument};

mod datetime;
mod github;
mod mutation;
mod photo;
mod query;
mod random_user;
mod simple_broker;
mod subscription;
mod tag;
mod user;

async fn graphql_handler(
    State(AppState { schema, database }): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    if let Some(token) = get_token_from_headers(&headers) {
        let collection = database.collection::<UserDocument>("users");
        let user = collection.find_one(doc! {"github_token": token.0}).await;
        if let Ok(Some(user)) = user {
            let user = User {
                name: user.name,
                avatar: user.avatar,
                github_login: user.github_login.into(),
            };
            request = request.data(user);
        }
    }
    schema.execute(request).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

struct Token(String);

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

#[derive(Clone)]
struct AppState {
    schema: Schema<QueryRoot, MutationRoot, SubscriptionRoot>,
    database: Database,
}

#[tokio::main]
async fn main() {
    let uri = dotenv::var("DB_HOST").unwrap();
    let client = Client::with_uri_str(uri).await.unwrap();
    let database = client.database("sample");

    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(database.clone())
        .finish();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(cors)
        .with_state(AppState { schema, database });
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
