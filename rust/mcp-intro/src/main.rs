use std::sync::OnceLock;

use anyhow::Result;
use rmcp::{
    Error as McpError, ServerHandler, ServiceExt,
    model::{CallToolResult, Content},
    tool,
    transport::stdio,
};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(JsonSchema, Debug)]
struct Artist {
    #[schemars(description = "アーティスト名")]
    name: String,
    #[schemars(description = "ジャンル")]
    genres: Vec<String>,
}

#[derive(Deserialize, JsonSchema)]
struct SearchQuery {
    #[schemars(description = "ジャンル")]
    genre: String,
}

static FAKE_ARTISTS: OnceLock<Vec<Artist>> = OnceLock::new();

fn get_fake_artists() -> &'static [Artist] {
    FAKE_ARTISTS.get_or_init(|| {
        vec![
            Artist {
                name: "Polygon Frame".to_string(),
                genres: vec!["math rock".to_string(), "instrumental".to_string()],
            },
            Artist {
                name: "Sakana Vortex".to_string(),
                genres: vec!["マスロック".to_string(), "ポストロック".to_string()],
            },
            Artist {
                name: "Neon Leaf".to_string(),
                genres: vec!["synthpop".to_string(), "electropop".to_string()],
            },
            Artist {
                name: "Midori Compass".to_string(),
                genres: vec!["マスロック".to_string(), "プログレ".to_string()],
            },
            Artist {
                name: "Cloudway Drive".to_string(),
                genres: vec!["shoegaze".to_string()],
            },
            Artist {
                name: "Crimson Echo".to_string(),
                genres: vec!["math rock".to_string(), "emo".to_string()],
            },
            Artist {
                name: "Silent Atlas".to_string(),
                genres: vec!["ambient".to_string()],
            },
            Artist {
                name: "Tokyo Tangle".to_string(),
                genres: vec!["ジャズロック".to_string()],
            },
            Artist {
                name: "Kaleido Bloom".to_string(),
                genres: vec!["マスロック".to_string(), "experimental".to_string()],
            },
            Artist {
                name: "Opal Ghost".to_string(),
                genres: vec!["dream pop".to_string(), "math rock".to_string()],
            },
        ]
    })
}

#[derive(Clone)]
pub struct ArtistSearch;

#[tool(tool_box)]
impl ArtistSearch {
    pub fn new() -> Self {
        Self
    }

    #[tool(description = "ジャンルに基づいてアーティストを検索します")]
    fn search(
        &self,
        #[tool(aggr)] SearchQuery { genre }: SearchQuery,
    ) -> Result<CallToolResult, McpError> {
        let results: Vec<_> = get_fake_artists().iter().collect();
        let output = if results.is_empty() {
            format!(
                "ジャンル '{}' に一致するアーティストが見つかりませんでした。",
                genre
            )
        } else {
            let mut output = format!("ジャンル '{}' の検索結果:\n\n", genre);
            for artist in results {
                output.push_str(&format!(
                    "アーティスト名: {}\nジャンル: {}",
                    artist.name,
                    artist.genres.join(",")
                ));
            }

            output
        };

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }
}

#[tool(tool_box)]
impl ServerHandler for ArtistSearch {}

#[tokio::main]
async fn main() -> Result<()> {
    let service = ArtistSearch::new().serve(stdio()).await?;
    service.waiting().await?;

    Ok(())
}
