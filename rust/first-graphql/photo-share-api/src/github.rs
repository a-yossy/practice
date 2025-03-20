use mongodb::bson::doc;
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GithubCredential {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
}

#[derive(Deserialize)]
struct GithubCredentialResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GithubUserAccountResponse {
    message: Option<String>,
    avatar_url: Option<String>,
    login: String,
    name: Option<String>,
}

pub struct GithubAuthorizeResponse {
    pub access_token: String,
    pub message: Option<String>,
    pub avatar_url: Option<String>,
    pub login: String,
    pub name: Option<String>,
}

async fn request_github_token(credential: GithubCredential) -> GithubCredentialResponse {
    let client = ReqwestClient::new();
    client
        .post("https://github.com/login/oauth/access_token")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&credential)
        .send()
        .await
        .unwrap()
        .json::<GithubCredentialResponse>()
        .await
        .unwrap()
}

async fn request_github_user_account(token: &String) -> GithubUserAccountResponse {
    let client = ReqwestClient::new();
    client
        .get("https://api.github.com/user")
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "rust-first-graphql")
        .send()
        .await
        .unwrap()
        .json::<GithubUserAccountResponse>()
        .await
        .unwrap()
}

pub async fn authorize_with_github(credential: GithubCredential) -> GithubAuthorizeResponse {
    let GithubCredentialResponse { access_token } = request_github_token(credential).await;
    let GithubUserAccountResponse {
        message,
        avatar_url,
        login,
        name,
    } = request_github_user_account(&access_token).await;

    GithubAuthorizeResponse {
        access_token,
        message,
        avatar_url,
        login,
        name,
    }
}
