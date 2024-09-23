use anyhow::Context;
use askama::Template;
use axum::{
    async_trait,
    extract::{FromRequestParts, Query, State},
    http::request::Parts,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use base64::Engine;
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use std::{net::SocketAddr, str::from_utf8};

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

struct User;

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = axum::http::Response<axum::body::Body>;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Basic ") {
                let credentials = auth_header.trim_start_matches("Basic ");
                let decoded = base64::prelude::BASE64_STANDARD
                    .decode(credentials)
                    .unwrap_or_default();
                let credential_str = from_utf8(&decoded).unwrap_or("");

                if credential_str == "test:test" {
                    return Ok(User);
                }
            }
        }

        let reject_response = axum::http::Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(
                "WWW-Authenticate",
                "Basic realm=\"Please enter your credentials\"",
            )
            .body(axum::body::Body::from("Unauthorized"))
            .unwrap();

        Err(reject_response)
    }
}

#[derive(Deserialize)]
pub struct GeoResponse {
    pub results: Vec<LatLong>,
}

#[derive(sqlx::FromRow, Deserialize, Debug, Clone)]
pub struct LatLong {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize)]
pub struct WeatherQuery {
    pub city: String,
}

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub hourly: Hourly,
}

#[derive(Deserialize, Debug)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
}

#[derive(Template, Deserialize, Debug)]
#[template(path = "weather.html")]
pub struct WeatherDisplay {
    pub city: String,
    pub forecasts: Vec<Forecast>,
}

impl WeatherDisplay {
    fn new(city: String, response: WeatherResponse) -> Self {
        let display = WeatherDisplay {
            city,
            forecasts: response
                .hourly
                .time
                .iter()
                .zip(response.hourly.temperature_2m.iter())
                .map(|(date, temperature)| Forecast {
                    date: date.to_string(),
                    temperature: temperature.to_string(),
                })
                .collect(),
        };
        display
    }
}

#[derive(Deserialize, Debug)]
pub struct Forecast {
    pub date: String,
    pub temperature: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "stats.html")]
struct StatsTemplate {
    pub cities: Vec<City>,
}

#[derive(sqlx::FromRow)]
pub struct City {
    pub name: String,
}

async fn fetch_lat_long(city: &str) -> Result<LatLong, anyhow::Error> {
    let endpoint = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
        city
    );
    let response = reqwest::get(&endpoint).await?.json::<GeoResponse>().await?;
    response.results.get(0).cloned().context("No results found")
}

async fn fetch_weather(lat_long: LatLong) -> Result<WeatherResponse, anyhow::Error> {
    let endpoint = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m",
        lat_long.latitude, lat_long.longitude
    );
    let response = reqwest::get(&endpoint)
        .await?
        .json::<WeatherResponse>()
        .await?;
    Ok(response)
}

async fn get_lat_long(pool: &PgPool, name: &str) -> Result<LatLong, anyhow::Error> {
    let lat_long = sqlx::query_as::<_, LatLong>(
        "SELECT lat AS latitude, long as longitude FROM cities WHERE name = $1",
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;

    if let Some(lat_long) = lat_long {
        return Ok(lat_long);
    }

    let lat_long = fetch_lat_long(name).await?;
    sqlx::query("INSERT INTO cities (name, lat, long) VALUES ($1, $2, $3)")
        .bind(name)
        .bind(lat_long.latitude)
        .bind(lat_long.longitude)
        .execute(pool)
        .await?;

    Ok(lat_long)
}

async fn get_last_cities(pool: &PgPool) -> Result<Vec<City>, anyhow::Error> {
    let cities = sqlx::query_as::<_, City>("SELECT name FROM cities ORDER BY id DESC LIMIT 10")
        .fetch_all(pool)
        .await?;
    Ok(cities)
}

async fn index() -> HtmlTemplate<IndexTemplate> {
    HtmlTemplate(IndexTemplate)
}

async fn weather(
    Query(params): Query<WeatherQuery>,
    State(pool): State<PgPool>,
) -> Result<HtmlTemplate<WeatherDisplay>, AppError> {
    let lat_long = get_lat_long(&pool, &params.city).await?;
    let weather = fetch_weather(lat_long).await?;
    Ok(HtmlTemplate(WeatherDisplay::new(params.city, weather)))
}

async fn stats(
    _user: User,
    State(pool): State<PgPool>,
) -> Result<HtmlTemplate<StatsTemplate>, AppError> {
    let cities = get_last_cities(&pool).await?;
    Ok(HtmlTemplate(StatsTemplate { cities }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_connection_str = dotenv::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let pool = sqlx::PgPool::connect(&db_connection_str)
        .await
        .context("can't connect to database")?;

    let app = Router::new()
        .route("/", get(index))
        .route("/weather", get(weather))
        .route("/stats", get(stats))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
