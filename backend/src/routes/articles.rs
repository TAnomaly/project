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
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub slug: String,
    pub author_id: String,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ArticleQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub author_id: Option<String>,
}

pub fn article_routes() -> Router<Database> {
    Router::new()
        .route("/", get(get_articles))
}

#[derive(Debug, Serialize)]
struct ArticlesResponse {
    success: bool,
    data: Vec<Article>,
    pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
struct PaginationInfo {
    page: u32,
    limit: u32,
    total: usize,
    pages: u32,
}

async fn get_articles(
    State(db): State<Database>,
    Query(params): Query<ArticleQuery>,
) -> Result<Json<ArticlesResponse>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    let articles = if let Some(author_id) = params.author_id {
        sqlx::query_as::<_, Article>(
            "SELECT * FROM posts WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(&author_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        sqlx::query_as::<_, Article>(
            "SELECT * FROM posts ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&db.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let total = articles.len();
    let response = ArticlesResponse {
        success: true,
        data: articles,
        pagination: PaginationInfo {
            page,
            limit,
            total,
            pages: 1,
        },
    };
    Ok(Json(response))
}
