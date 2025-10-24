use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::Database;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Podcast {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub creator_id: String,
    pub episode_count: Option<i32>,
    pub total_duration: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PodcastQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub creator_id: Option<String>,
}

pub fn podcast_routes() -> Router<Database> {
    Router::new()
        .route("/", get(get_podcasts))
}

#[derive(Debug, Serialize)]
struct PodcastsResponse {
    success: bool,
    data: Vec<Podcast>,
    pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
struct PaginationInfo {
    page: u32,
    limit: u32,
    total: usize,
    pages: u32,
}

async fn get_podcasts(
    State(db): State<Database>,
    Query(params): Query<PodcastQuery>,
) -> Result<Json<PodcastsResponse>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    // Mock data for now since we don't have a podcasts table
    let podcasts = vec![
        Podcast {
            id: Uuid::new_v4(),
            title: "Sample Podcast".to_string(),
            description: Some("A sample podcast episode".to_string()),
            creator_id: params.creator_id.unwrap_or_default(),
            episode_count: Some(5),
            total_duration: Some("2h 30m".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    ];

    let total = podcasts.len();
    let response = PodcastsResponse {
        success: true,
        data: podcasts,
        pagination: PaginationInfo {
            page,
            limit,
            total,
            pages: 1,
        },
    };
    Ok(Json(response))
}
