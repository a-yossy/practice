use reqwest::Client as ReqwestClient;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RandomUserLoginResponse {
    pub username: String,
    pub sha1: String,
}
#[derive(Deserialize)]
pub struct RandomUserNameResponse {
    pub first: String,
    pub last: String,
}
#[derive(Deserialize)]
pub struct RandomUserPictureResponse {
    pub thumbnail: String,
}
#[derive(Deserialize)]
pub struct RandomUserResponseBody {
    pub login: RandomUserLoginResponse,
    pub name: RandomUserNameResponse,
    pub picture: RandomUserPictureResponse,
}
#[derive(Deserialize)]
pub struct RandomUserResponse {
    pub results: Vec<RandomUserResponseBody>,
}

pub async fn random_user(count: usize) -> RandomUserResponse {
    let random_user_api = format!("https://randomuser.me/api/?results={}", count);
    let client = ReqwestClient::new();
    client
        .get(random_user_api)
        .send()
        .await
        .unwrap()
        .json::<RandomUserResponse>()
        .await
        .unwrap()
}
